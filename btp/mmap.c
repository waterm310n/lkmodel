#include <stdio.h>
#include <stdlib.h>
#include <sys/mount.h>

static int mount_procfs()
{
    if (mount("proc", "/proc", "proc", 0, NULL) != 0) {
        printf("mount proc error!\n");
        return -1;
    }
}

int main()
{
    printf("Hello, mmap!\n");

    if (mount_procfs() < 0) {
        exit(-1);
    }

    FILE *fp = fopen("/proc/self/maps", "r");

    while (1) {
        char buf[256];
        if (fgets(buf, sizeof(buf), fp) == NULL) {
            break;
        }
        printf("%s", buf);
    }

    fclose(fp);
    return 0;
}
