mod cuda_foo;

fn main()
{
	unsafe
	{
		println!("test: {}", cuda_foo::foo(1));
	}
}
