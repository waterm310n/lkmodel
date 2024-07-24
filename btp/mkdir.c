#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/mount.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <fcntl.h>
#include <errno.h>

void err_reason(int err)
{
    switch (errno) {
        case EACCES:
            printf("Permission denied: %s\n", strerror(errno));
            break;
        case EEXIST:
            printf("Directory already exists: %s\n", strerror(errno));
            break;
        case ENOSPC:
            printf("No space left on device: %s\n", strerror(errno));
            break;
        case EROFS:
            printf("Read-only file system: %s\n", strerror(errno));
            break;
        case ENOTDIR:
            printf("A component of the path is not a directory: %s\n", strerror(errno));
            break;
        case ENAMETOOLONG:
            printf("Path name is too long: %s\n", strerror(errno));
            break;
        default:
            printf("An unknown error occurred: %s\n", strerror(errno));
            break;
    }
}

int main(int argc, char *argv[]) {
    const char *target = "/abc";

    int i = 0;
    for (i = 0; i < 2; i++) {
        int ret = mkdir(target, 0755);
        if (ret == -1) {
            printf("mkdir %s error!\n", target);
            err_reason(ret);
        } else {
            printf("mkdir %s ok!\n", target);
        }
    }
    return 0;
}
