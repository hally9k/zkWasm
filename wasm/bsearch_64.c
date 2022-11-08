#include "bsearch.h"

unsigned long long wasm_input(int);

static inline unsigned long long wasm_public_input()
{
	return wasm_input(1);
}

static inline unsigned long long wasm_private_input()
{
	return wasm_input(0);
}

__attribute__((visibility("default")))
unsigned int bsearch()
{
	unsigned int l = 0;
	unsigned int r = RIGHT;
	unsigned int mid;

	unsigned long long v = wasm_input(1);

	while (l != r)
	{
		mid = (l + r) / 2;

		if (array[mid] < v)
		{
			l = mid + 1;
		}
		else
		{
			r = mid;
		}
	}

	if (array[l] == v)
	{
		return l;
	}
	return FAILED;
}
