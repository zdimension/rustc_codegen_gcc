use gccjit::{Context, FunctionType, OutputKind};

fn main() {
    let context = Context::default();
    let int_type = context.new_type::<i32>();
    let function = context.new_function(None, FunctionType::Exported, int_type, &[], "main", false);
    let block = function.new_block("block");
    block.end_with_return(None, context.new_rvalue_from_int(int_type, 0));
    context.compile_to_file(OutputKind::Executable, "main-m68k");
}
