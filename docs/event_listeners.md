Ui::listen
Ui::listen_once

Ui::listen_global
Ui::listen_global_once

Ui::listen_while
Ui::listen_global_while

Ui::listen_off

Ui::send_to
Ui::send_global

Nested listeners work, including Ui::listen_off
Mutable state in closures works without any Rc/Cell/RefCell/Mutex
