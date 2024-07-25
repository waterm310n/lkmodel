#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>

void init_mount() {
    printf("init_mount\n");

    mkdir("/proc", 0755);

    pid_t pid = vfork();
    if (pid == 0) {
        execl("/sbin/mount", "mount", "-a", NULL);
        exit(0);
    }
    int ret = 0;
    waitpid(pid, &ret, 0);
}

int main(int argc, char *argv[])
{
    printf("Hello, init!\n");

    init_mount();
    printf("init mount successfully\n");

    pid_t pid = vfork();
    if (pid == 0) {
        execl("/sbin/procfs", "procfs" ,NULL);
        exit(0);
    }

    int ret = 0;
    waitpid(pid, &ret, 0);

    return 0;
}