#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/mount.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <errno.h>

int main(int argc, char *argv[]) {
    const char *source = "proc";
    const char *target = "/proc";

    // mkdir /proc
    if (mkdir(target, 0755) == -1) {
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

    // mount proc filesystem
    if (mount(source, target, "proc", MS_MGC_VAL, NULL) == -1) {
        perror("mount");
        return 1;
    }

    printf("File system mounted successfully\n");

    pid_t pid = vfork();
    if (pid == 0) {
        printf("Child is running ...\n");
        execl("/sbin/procfs", "procfs", NULL);
        exit(0);
    } else {
        int ret = 0;
        waitpid(pid, &ret, 0);
        printf("Parent gets code [%d]\n", ret);
    }
    return 0;
}