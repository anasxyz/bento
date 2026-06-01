use crate::{Group, Ui};

impl Ui {
    pub(crate) fn layout_node(&mut self, id: usize, available_w: f32, available_h: f32) {
        let children = match self.nodes[id].as_ref() {
            Some(n) => n.children.clone(),
            None => return,
        };

        let layout_info = match self.nodes[id].as_ref() {
            Some(n) => n
                .widget
                .as_any()
                .downcast_ref::<Group>()
                .map(|g| g.layout.clone()),
            None => return,
        };

        let width_sizing = self.nodes[id]
            .as_ref()
            .unwrap()
            .widget
            .width_sizing()
            .clone();
        let height_sizing = self.nodes[id]
            .as_ref()
            .unwrap()
            .widget
            .height_sizing()
            .clone();

        let inner_w = match &width_sizing {
            Size::Auto => available_w,
            s => s.resolve(available_w),
        };
        let inner_h = match &height_sizing {
            Size::Auto => available_h,
            s => s.resolve(available_h),
        };

        if let Some(Some(node)) = self.nodes.get_mut(id) {
            if let Some(g) = node.widget.as_any_mut().downcast_mut::<Group>() {
                if !matches!(g.width, Size::Auto) {
                    g.w = inner_w;
                }
                if !matches!(g.height, Size::Auto) {
                    g.h = inner_h;
                }
            }
        }

        for child_id in &children {
            if let Some(Some(node)) = self.nodes.get(*child_id) {
                let ws = node.widget.width_sizing().clone();
                let hs = node.widget.height_sizing().clone();
                let needs_w = !ws.is_auto() && !matches!(ws, Size::Fill);
                let needs_h = !hs.is_auto() && !matches!(hs, Size::Fill);
                if needs_w && needs_h {
                    let new_w = ws.resolve(inner_w);
                    let new_h = hs.resolve(inner_h);
                    if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                        node.widget.set_size(new_w, new_h);
                    }
                } else if needs_w && !hs.is_auto() {
                    let cur = node.widget.size();
                    let new_w = ws.resolve(inner_w);
                    if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                        node.widget.set_size(new_w, cur.1);
                    }
                } else if needs_h && !ws.is_auto() {
                    let cur = node.widget.size();
                    let new_h = hs.resolve(inner_h);
                    if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                        node.widget.set_size(cur.0, new_h);
                    }
                }
            }
        }

        match layout_info {
            None | Some(Layout::None) => {
                for child_id in &children {
                    if let Some(Some(node)) = self.nodes.get(*child_id) {
                        let ws = node.widget.width_sizing().clone();
                        let hs = node.widget.height_sizing().clone();
                        if matches!(ws, Size::Fill) || matches!(hs, Size::Fill) {
                            let cur = node.widget.size();
                            let new_w = if matches!(ws, Size::Fill) {
                                inner_w
                            } else {
                                cur.0
                            };
                            let new_h = if matches!(hs, Size::Fill) {
                                inner_h
                            } else {
                                cur.1
                            };
                            if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                                node.widget.set_size(new_w, new_h);
                            }
                        }
                    }
                }
                for child_id in children {
                    if self.layout_dirty.contains(&child_id) {
                        self.layout_node(child_id, inner_w, inner_h);
                    }
                }
            }

            Some(Layout::Row {
                gap,
                padding,
                main_axis,
                cross_axis,
                wrap,
            }) => {
                let pad_left = padding[3];
                let pad_right = padding[1];
                let pad_top = padding[0];
                let pad_bottom = padding[2];
                let avail_w = inner_w - pad_left - pad_right;
                let avail_h = inner_h - pad_top - pad_bottom;

                // pass 0: always recurse auto-width children to measure them
                for child_id in &children {
                    if let Some(Some(node)) = self.nodes.get(*child_id) {
                        if matches!(node.widget.width_sizing(), Size::Auto) {
                            self.layout_node(*child_id, avail_w, avail_h);
                        }
                    }
                }

                if wrap {
                    let mut lines: Vec<Vec<usize>> = Vec::new();
                    let mut current_line: Vec<usize> = Vec::new();
                    let mut line_w = 0.0f32;

                    for child_id in &children {
                        if let Some(Some(node)) = self.nodes.get(*child_id) {
                            let cw = node.widget.size().0;
                            let needed = if current_line.is_empty() {
                                cw
                            } else {
                                cw + gap
                            };
                            if !current_line.is_empty() && line_w + needed > avail_w {
                                lines.push(std::mem::take(&mut current_line));
                                line_w = cw;
                                current_line.push(*child_id);
                            } else {
                                line_w += needed;
                                current_line.push(*child_id);
                            }
                        }
                    }
                    if !current_line.is_empty() {
                        lines.push(current_line);
                    }

                    let all_lines_h: f32 = {
                        let mut h = 0.0f32;
                        for line in &lines {
                            let line_h = line
                                .iter()
                                .filter_map(|id| {
                                    self.nodes.get(*id)?.as_ref().map(|n| n.widget.size().1)
                                })
                                .fold(0.0f32, f32::max);
                            h += line_h + gap;
                        }
                        h
                    };

                    let y_offset = match cross_axis {
                        CrossAxis::Start => pad_top,
                        CrossAxis::Center => pad_top + (avail_h - all_lines_h) / 2.0,
                        CrossAxis::End => pad_top + avail_h - all_lines_h,
                        CrossAxis::Stretch => pad_top,
                    };

                    let mut y_cursor = y_offset;
                    let mut total_h = 0.0f32;
                    for line in &lines {
                        let line_h = line
                            .iter()
                            .filter_map(|id| {
                                self.nodes.get(*id)?.as_ref().map(|n| n.widget.size().1)
                            })
                            .fold(0.0f32, f32::max);

                        let line_content_w: f32 = line
                            .iter()
                            .filter_map(|id| {
                                self.nodes.get(*id)?.as_ref().map(|n| n.widget.size().0)
                            })
                            .sum::<f32>()
                            + gap * (line.len().saturating_sub(1)) as f32;

                        let x_start = match main_axis {
                            MainAxis::Start => pad_left,
                            MainAxis::Center => pad_left + (avail_w - line_content_w) / 2.0,
                            MainAxis::End => pad_left + avail_w - line_content_w,
                            MainAxis::SpaceBetween => pad_left,
                            MainAxis::SpaceAround => pad_left,
                        };

                        let between_gap = match main_axis {
                            MainAxis::SpaceBetween if line.len() > 1 => {
                                (avail_w - line_content_w
                                    + gap * (line.len().saturating_sub(1)) as f32)
                                    / (line.len().saturating_sub(1)) as f32
                            }
                            MainAxis::SpaceAround => {
                                (avail_w
                                    - (line_content_w
                                        - gap * (line.len().saturating_sub(1)) as f32))
                                    / line.len() as f32
                            }
                            _ => gap,
                        };

                        let x_start = match main_axis {
                            MainAxis::SpaceAround => pad_left + between_gap / 2.0,
                            _ => x_start,
                        };

                        let mut x_cursor = x_start;
                        for child_id in line {
                            let (cw, ch) = self.nodes[*child_id]
                                .as_ref()
                                .map(|n| n.widget.size())
                                .unwrap_or((0.0, 0.0));
                            let cy = match cross_axis {
                                CrossAxis::Start => y_cursor,
                                CrossAxis::Center => y_cursor + (line_h - ch) / 2.0,
                                CrossAxis::End => y_cursor + line_h - ch,
                                CrossAxis::Stretch => {
                                    if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                                        node.widget.set_size(cw, line_h);
                                    }
                                    y_cursor
                                }
                            };
                            if let Some(Some(n)) = self.nodes.get_mut(*child_id) {
                                n.widget.set_position(x_cursor, cy);
                            }
                            if self.layout_dirty.contains(child_id) {
                                self.layout_node(*child_id, cw, line_h);
                            }
                            x_cursor += cw + between_gap;
                        }
                        y_cursor += line_h + gap;
                        total_h += line_h + gap;
                    }

                    if let Some(Some(node)) = self.nodes.get_mut(id) {
                        if let Some(g) = node.widget.as_any_mut().downcast_mut::<Group>() {
                            if matches!(g.height, Size::Auto) {
                                g.h = total_h + pad_top + pad_bottom;
                            }
                        }
                    }
                } else {
                    let is_auto_w = matches!(width_sizing, Size::Auto);
                    let mut fixed_total = 0.0f32;
                    let mut fill_count = 0;
                    let child_count = children.len();

                    for child_id in &children {
                        if let Some(Some(node)) = self.nodes.get(*child_id) {
                            match node.widget.width_sizing() {
                                Size::Fill => fill_count += 1,
                                _ => fixed_total += node.widget.size().0,
                            }
                        }
                    }

                    let total_gap = gap * (child_count.saturating_sub(1)) as f32;
                    let content_w = if is_auto_w {
                        fixed_total + total_gap
                    } else {
                        avail_w
                    };
                    let remaining = (content_w - fixed_total - total_gap).max(0.0);
                    let fill_w = if fill_count > 0 {
                        remaining / fill_count as f32
                    } else {
                        0.0
                    };

                    for child_id in &children {
                        if let Some(Some(node)) = self.nodes.get(*child_id) {
                            let set_w = matches!(node.widget.width_sizing(), Size::Fill);
                            let set_h = matches!(node.widget.height_sizing(), Size::Fill);
                            if set_w || set_h {
                                let cur = node.widget.size();
                                let new_w = if set_w { fill_w } else { cur.0 };
                                let new_h = if set_h { avail_h } else { cur.1 };
                                if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                                    node.widget.set_size(new_w, new_h);
                                }
                            }
                        }
                    }

                    let child_sizes: Vec<(f32, f32)> = children
                        .iter()
                        .map(|id| {
                            self.nodes[*id]
                                .as_ref()
                                .map(|n| n.widget.size())
                                .unwrap_or((0.0, 0.0))
                        })
                        .collect();

                    let total_content_w: f32 = child_sizes.iter().map(|(w, _)| w).sum::<f32>()
                        + gap * (child_count.saturating_sub(1)) as f32;

                    let (x_start, between_gap) = match main_axis {
                        MainAxis::Start => (pad_left, gap),
                        MainAxis::Center => (pad_left + (avail_w - total_content_w) / 2.0, gap),
                        MainAxis::End => (pad_left + avail_w - total_content_w, gap),
                        MainAxis::SpaceBetween => {
                            let g = if child_count > 1 {
                                (avail_w - child_sizes.iter().map(|(w, _)| w).sum::<f32>())
                                    / (child_count.saturating_sub(1)) as f32
                            } else {
                                0.0
                            };
                            (pad_left, g)
                        }
                        MainAxis::SpaceAround => {
                            let total_cw: f32 = child_sizes.iter().map(|(w, _)| w).sum();
                            let space = (avail_w - total_cw) / child_count as f32;
                            (pad_left + space / 2.0, space)
                        }
                    };

                    let mut cursor = x_start;
                    for (i, child_id) in children.iter().enumerate() {
                        let (cw, ch) = child_sizes[i];
                        let cross_avail_h = inner_h - pad_top - pad_bottom;
                        let cy = match cross_axis {
                            CrossAxis::Start => pad_top,
                            CrossAxis::Center => pad_top + (cross_avail_h - ch) / 2.0,
                            CrossAxis::End => pad_top + cross_avail_h - ch,
                            CrossAxis::Stretch => {
                                if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                                    node.widget.set_size(cw, cross_avail_h);
                                }
                                pad_top
                            }
                        };
                        if let Some(Some(n)) = self.nodes.get_mut(*child_id) {
                            n.widget.set_position(cursor, cy);
                        }
                        cursor += cw + between_gap;
                        if self.layout_dirty.contains(child_id) {
                            self.layout_node(*child_id, cw, avail_h);
                        }
                    }

                    let mut total_w = 0.0f32;
                    let mut total_h = 0.0f32;
                    for child_id in &children {
                        if let Some(n) = self.nodes[*child_id].as_ref() {
                            let (cw, ch) = n.widget.size();
                            total_w += cw + gap;
                            total_h = total_h.max(ch);
                        }
                    }
                    if let Some(n) = self.nodes[id].as_mut() {
                        if let Some(g) = n.widget.as_any_mut().downcast_mut::<Group>() {
                            let final_w = if matches!(g.width, Size::Auto) {
                                total_w + pad_left + pad_right
                            } else {
                                g.w
                            };
                            let final_h = if matches!(g.height, Size::Auto) {
                                total_h + pad_top + pad_bottom
                            } else {
                                g.h
                            };
                            let size_changed = g.w != final_w || g.h != final_h;
                            g.w = final_w;
                            g.h = final_h;
                            if size_changed {
                                if let Some(parent_id) =
                                    self.nodes[id].as_ref().and_then(|n| n.parent)
                                {
                                    self.layout_dirty.insert(parent_id);
                                }
                            }
                        }
                    }
                }
            }

            Some(Layout::Column {
                gap,
                padding,
                main_axis,
                cross_axis,
                wrap,
            }) => {
                let pad_left = padding[3];
                let pad_right = padding[1];
                let pad_top = padding[0];
                let pad_bottom = padding[2];
                let avail_w = inner_w - pad_left - pad_right;
                let avail_h = inner_h - pad_top - pad_bottom;

                let is_auto_h = matches!(height_sizing, Size::Auto);

                // pass 0: always recurse auto-height children to measure them
                if is_auto_h {
                    for child_id in &children {
                        if let Some(Some(node)) = self.nodes.get(*child_id) {
                            if matches!(node.widget.height_sizing(), Size::Auto) {
                                self.layout_node(*child_id, avail_w, avail_h);
                            }
                        }
                    }
                }

                if wrap {
                    let mut cols: Vec<Vec<usize>> = Vec::new();
                    let mut current_col: Vec<usize> = Vec::new();
                    let mut col_h = 0.0f32;

                    for child_id in &children {
                        if let Some(Some(node)) = self.nodes.get(*child_id) {
                            let ch = node.widget.size().1;
                            let needed = if current_col.is_empty() { ch } else { ch + gap };
                            if !current_col.is_empty() && col_h + needed > avail_h {
                                cols.push(std::mem::take(&mut current_col));
                                col_h = ch;
                                current_col.push(*child_id);
                            } else {
                                col_h += needed;
                                current_col.push(*child_id);
                            }
                        }
                    }
                    if !current_col.is_empty() {
                        cols.push(current_col);
                    }

                    let all_cols_w: f32 = {
                        let mut w = 0.0f32;
                        for col in &cols {
                            let col_w = col
                                .iter()
                                .filter_map(|id| {
                                    self.nodes.get(*id)?.as_ref().map(|n| n.widget.size().0)
                                })
                                .fold(0.0f32, f32::max);
                            w += col_w + gap;
                        }
                        w
                    };

                    let x_offset = match cross_axis {
                        CrossAxis::Start => pad_left,
                        CrossAxis::Center => pad_left + (avail_w - all_cols_w) / 2.0,
                        CrossAxis::End => pad_left + avail_w - all_cols_w,
                        CrossAxis::Stretch => pad_left,
                    };

                    let mut x_cursor = x_offset;
                    let mut total_w = 0.0f32;
                    for col in &cols {
                        let col_w = col
                            .iter()
                            .filter_map(|id| {
                                self.nodes.get(*id)?.as_ref().map(|n| n.widget.size().0)
                            })
                            .fold(0.0f32, f32::max);

                        let col_content_h: f32 = col
                            .iter()
                            .filter_map(|id| {
                                self.nodes.get(*id)?.as_ref().map(|n| n.widget.size().1)
                            })
                            .sum::<f32>()
                            + gap * (col.len().saturating_sub(1)) as f32;

                        let y_start = match main_axis {
                            MainAxis::Start => pad_top,
                            MainAxis::Center => pad_top + (avail_h - col_content_h) / 2.0,
                            MainAxis::End => pad_top + avail_h - col_content_h,
                            MainAxis::SpaceBetween => pad_top,
                            MainAxis::SpaceAround => pad_top,
                        };

                        let between_gap = match main_axis {
                            MainAxis::SpaceBetween if col.len() > 1 => {
                                (avail_h - col_content_h
                                    + gap * (col.len().saturating_sub(1)) as f32)
                                    / (col.len().saturating_sub(1)) as f32
                            }
                            MainAxis::SpaceAround => {
                                (avail_h
                                    - (col_content_h - gap * (col.len().saturating_sub(1)) as f32))
                                    / col.len() as f32
                            }
                            _ => gap,
                        };

                        let y_start = match main_axis {
                            MainAxis::SpaceAround => pad_top + between_gap / 2.0,
                            _ => y_start,
                        };

                        let mut y_cursor = y_start;
                        for child_id in col {
                            let (cw, ch) = self.nodes[*child_id]
                                .as_ref()
                                .map(|n| n.widget.size())
                                .unwrap_or((0.0, 0.0));
                            let cx = match cross_axis {
                                CrossAxis::Start => x_cursor,
                                CrossAxis::Center => x_cursor + (col_w - cw) / 2.0,
                                CrossAxis::End => x_cursor + col_w - cw,
                                CrossAxis::Stretch => {
                                    if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                                        node.widget.set_size(col_w, ch);
                                    }
                                    x_cursor
                                }
                            };
                            if let Some(Some(n)) = self.nodes.get_mut(*child_id) {
                                n.widget.set_position(cx, y_cursor);
                            }
                            if self.layout_dirty.contains(child_id) {
                                self.layout_node(*child_id, col_w, ch);
                            }
                            y_cursor += ch + between_gap;
                        }
                        x_cursor += col_w + gap;
                        total_w += col_w + gap;
                    }

                    if let Some(Some(node)) = self.nodes.get_mut(id) {
                        if let Some(g) = node.widget.as_any_mut().downcast_mut::<Group>() {
                            if matches!(g.width, Size::Auto) {
                                g.w = total_w + pad_left + pad_right;
                            }
                        }
                    }
                } else {
                    let mut fixed_total = 0.0f32;
                    let mut fill_count = 0;
                    let child_count = children.len();

                    for child_id in &children {
                        if let Some(Some(node)) = self.nodes.get(*child_id) {
                            match node.widget.height_sizing() {
                                Size::Fill => fill_count += 1,
                                _ => fixed_total += node.widget.size().1,
                            }
                        }
                    }

                    let total_gap = gap * (child_count.saturating_sub(1)) as f32;
                    let content_h = if is_auto_h {
                        fixed_total + total_gap
                    } else {
                        avail_h
                    };
                    let remaining = (content_h - fixed_total - total_gap).max(0.0);
                    let fill_h = if fill_count > 0 {
                        remaining / fill_count as f32
                    } else {
                        0.0
                    };

                    for child_id in &children {
                        if let Some(Some(node)) = self.nodes.get(*child_id) {
                            let set_w = matches!(node.widget.width_sizing(), Size::Fill);
                            let set_h = matches!(node.widget.height_sizing(), Size::Fill);
                            if set_w || set_h {
                                let cur = node.widget.size();
                                let fill_w = if matches!(width_sizing, Size::Auto) {
                                    let mut max_w = 0.0f32;
                                    for cid in &children {
                                        if let Some(Some(cn)) = self.nodes.get(*cid) {
                                            if !matches!(cn.widget.width_sizing(), Size::Fill) {
                                                max_w = max_w.max(cn.widget.size().0);
                                            }
                                        }
                                    }
                                    max_w
                                } else {
                                    avail_w
                                };
                                let new_w = if set_w { fill_w } else { cur.0 };
                                let new_h = if set_h && !is_auto_h { fill_h } else { cur.1 };
                                if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                                    node.widget.set_size(new_w, new_h);
                                }
                            }
                        }
                    }

                    let child_sizes: Vec<(f32, f32)> = children
                        .iter()
                        .map(|id| {
                            self.nodes[*id]
                                .as_ref()
                                .map(|n| n.widget.size())
                                .unwrap_or((0.0, 0.0))
                        })
                        .collect();

                    let total_content_h: f32 = child_sizes.iter().map(|(_, h)| h).sum::<f32>()
                        + gap * (child_count.saturating_sub(1)) as f32;

                    let (y_start, between_gap) = match main_axis {
                        MainAxis::Start => (pad_top, gap),
                        MainAxis::Center => (pad_top + (avail_h - total_content_h) / 2.0, gap),
                        MainAxis::End => (pad_top + avail_h - total_content_h, gap),
                        MainAxis::SpaceBetween => {
                            let g = if child_count > 1 {
                                (avail_h - child_sizes.iter().map(|(_, h)| h).sum::<f32>())
                                    / (child_count.saturating_sub(1)) as f32
                            } else {
                                0.0
                            };
                            (pad_top, g)
                        }
                        MainAxis::SpaceAround => {
                            let total_ch: f32 = child_sizes.iter().map(|(_, h)| h).sum();
                            let space = (avail_h - total_ch) / child_count as f32;
                            (pad_top + space / 2.0, space)
                        }
                    };

                    let mut cursor = y_start;
                    for (i, child_id) in children.iter().enumerate() {
                        let (cw, ch) = child_sizes[i];
                        let cross_avail_w = inner_w - pad_left - pad_right;
                        let cx = match cross_axis {
                            CrossAxis::Start => pad_left,
                            CrossAxis::Center => pad_left + (cross_avail_w - cw) / 2.0,
                            CrossAxis::End => pad_left + cross_avail_w - cw,
                            CrossAxis::Stretch => {
                                if let Some(Some(node)) = self.nodes.get_mut(*child_id) {
                                    node.widget.set_size(cross_avail_w, ch);
                                }
                                pad_left
                            }
                        };
                        if let Some(Some(n)) = self.nodes.get_mut(*child_id) {
                            n.widget.set_position(cx, cursor);
                        }
                        cursor += ch + between_gap;
                        if self.layout_dirty.contains(child_id) {
                            self.layout_node(*child_id, avail_w, ch);
                        }
                    }

                    let mut total_w = 0.0f32;
                    let mut total_h = 0.0f32;
                    for child_id in &children {
                        if let Some(n) = self.nodes[*child_id].as_ref() {
                            let (cw, ch) = n.widget.size();
                            total_w = total_w.max(cw);
                            total_h += ch + gap;
                        }
                    }
                    if let Some(n) = self.nodes[id].as_mut() {
                        if let Some(g) = n.widget.as_any_mut().downcast_mut::<Group>() {
                            let final_w = if matches!(g.width, Size::Auto) {
                                total_w + pad_left + pad_right
                            } else {
                                g.w
                            };
                            let final_h = if matches!(g.height, Size::Auto) {
                                total_h + pad_top + pad_bottom
                            } else {
                                g.h
                            };
                            let size_changed = g.w != final_w || g.h != final_h;
                            g.w = final_w;
                            g.h = final_h;
                            if size_changed {
                                if let Some(parent_id) =
                                    self.nodes[id].as_ref().and_then(|n| n.parent)
                                {
                                    self.layout_dirty.insert(parent_id);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum Layout {
    None,
    Row {
        gap: f32,
        padding: [f32; 4],
        main_axis: MainAxis,
        cross_axis: CrossAxis,
        wrap: bool,
    },
    Column {
        gap: f32,
        padding: [f32; 4],
        main_axis: MainAxis,
        cross_axis: CrossAxis,
        wrap: bool,
    },
}

impl Default for Layout {
    fn default() -> Self {
        Layout::None
    }
}

#[derive(Clone, Default)]
pub enum MainAxis {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

#[derive(Clone, Default)]
pub enum CrossAxis {
    #[default]
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Debug)]
pub enum Size {
    Auto,
    Fixed(f32),
    Fill,
    Percent(f32),
    FillMinus(f32),
}

impl Size {
    pub fn resolve(&self, available: f32) -> f32 {
        match self {
            // placeholder for Auto, layout should not call resolve on it
            Size::Auto => 0.0,
            Size::Fixed(v) => *v,
            Size::Fill => available,
            Size::Percent(p) => available * p / 100.0,
            Size::FillMinus(v) => (available - v).max(0.0),
        }
    }

    pub fn is_auto(&self) -> bool {
        matches!(self, Size::Auto)
    }
}
