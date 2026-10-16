mod condition;
mod io;
mod loops;
mod vars;

fn main() {
    vars::run();
    io::run();
    condition::run();
    loops::run();
}
