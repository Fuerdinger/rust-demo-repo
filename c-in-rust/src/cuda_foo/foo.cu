#include "foo.h"
#include <cuda_runtime_api.h>

__global__ void k_foo(int32_t* val)
{
	*val += 1;
}

int32_t foo(int32_t input)
{
	int32_t* buf;
	cudaMalloc((void**)&buf, sizeof(int32_t));
	cudaMemcpy(buf, &input, sizeof(int32_t), cudaMemcpyHostToDevice);
	k_foo<<<1,1>>>(buf);
	cudaMemcpy(&input, buf, sizeof(int32_t), cudaMemcpyDeviceToHost);
	cudaFree(buf);
	return input;
}
