#[[ce_use(t.hce)]]
#define panic(func, file, line, res, errno) printf("Panic in %s at %s:%d: got %d as %s (errno: %d)\n", #func, file, line, res, strerror(errno), errno)
#define log(func, file, line, res, errno) printf("Log in %s at %s:%d: got %d as %s (errno: %d)\n", #func, file, line, res, strerror(errno), errno)
typedef type int;
type func_t()
{
	return 1;
}
type func_f()
{
	return 0;
}
void t_f()
{
	printf("Always fail:\n");
	func_f() _CE_DFT;
	func_f() _CE_PAN;
	func_f() _CE_PAN, _CE_DFT;
	func_f() _CE_PAN
	{
		printf("panic\n");
	}
	func_f() _CE_PAN
	{
		printf("panic\n");
	}
	_CE_DFT
	{
		printf("log\n");
	}
}
void t_t()
{
	printf("Always success:\n");
	func_t() _CE_DFT;
	func_t() _CE_PAN;
	func_t() _CE_PAN, _CE_DFT;
	func_t() _CE_PAN
	{
		printf("panic\n");
	}
	func_t() _CE_PAN
	{
		printf("panic\n");
	}
	_CE_DFT
	{
		printf("log\n");
	}
}
int main()
{
	t_t();
	t_f();
}