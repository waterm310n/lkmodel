#include <stdio.h>
#include <string.h>

#define LINELEN 512

int main()
{
	int ret;
	char line[LINELEN];
	FILE *fstatus = NULL;
    unsigned int lock_sz = 0;

	fstatus = fopen("/proc/self/status", "r");
	if (fstatus == NULL)
        printf("Open dev status failed\n");

	while (fgets(line, LINELEN, fstatus) != NULL)
		if (strstr(line, "VmLck") != NULL)
			break;

    printf("line: %s\n", line);
	ret = sscanf(line, "%*[^0-9]%d%*[^0-9]", &lock_sz);
	if (ret != 1)
		printf("Get lock size failed\n");

    printf("lock_sz: %u\n", lock_sz);
	fclose(fstatus);
    return 0;
}
