mod redgraph;

use inkwell::{context::Context, module::Module, builder::Builder, execution_engine::{ExecutionEngine, JitFunction}, passes::PassManagerSubType};
use petgraph::{Directed, stable_graph::StableGraph};
use redgraph::{node::Node, link::Link};

pub(crate) type Graph = StableGraph<Node, Link, Directed>;

pub type RedGraphJITFunction = unsafe extern "C" fn([u8]) -> [u8];

pub(crate) struct RedJITCompiler<'ctx> {
	llvm_context: &'ctx Context,
	llvm_module: Module<'ctx>,
	inkwell_builder: Builder<'ctx>,
	jit_execution_engine: Option<ExecutionEngine<'ctx>>,

	input_count: u32,
	output_count: u32,
	graph: Graph
}

impl<'ctx> RedJITCompiler<'ctx> {
	fn new(llvm_context: &'ctx Context, in_count: u32, out_count: u32) -> RedJITCompiler<'ctx> {
		RedJITCompiler { 
			llvm_context: &llvm_context,
			llvm_module: llvm_context.create_module("redjit_compiled_module"),
			inkwell_builder: llvm_context.create_builder(),
			jit_execution_engine: None,

			input_count: in_count,
			output_count: out_count,
			graph: Graph::new()
		}
	}
}

#[cfg(test)]
mod tests {
	use std::ops::Index;

use inkwell::values::IntValue;

use super::*;

	#[test]
	fn it_works() {
		let context = Context::create();

		let compiler = RedJITCompiler::new(&context, 2, 1);

		let i8_type = compiler.llvm_context.i8_type();
		let output_array_type = i8_type.array_type(compiler.output_count as u32);

		let fn_type = output_array_type.fn_type(&[i8_type.into()], true);
		let function = compiler.llvm_module.add_function("redgraph", fn_type, None);
		let fn_block = compiler.llvm_context.append_basic_block(function, "entry");

		compiler.inkwell_builder.position_at_end(fn_block);

		let tick = function.get_nth_param(0).unwrap().into_int_value();
		let mut circuit_inputs = Vec::<IntValue>::with_capacity(compiler.input_count as usize);

		for i in 1..compiler.input_count {
			circuit_inputs.push(function.get_nth_param(i).unwrap().into_int_value());	
		}

		for node_index in compiler.graph.node_indices() {
			let node = compiler.graph.index(node_index);
			let edges = compiler.graph.edges(node_index);
		};

	}
}
