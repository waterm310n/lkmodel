#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>
#include <unistd.h>
#include <errno.h>

static int test_zero()
{
    FILE *fp;
    char buf[32] = {1};

    printf("Test /dev/zero ...\n");
    fp = fopen("/dev/zero", "r+");
    if (fp == NULL) {
        fprintf(stderr, "open /dev/zero error: %d\n", errno);
        return -1;
    }

    memset(buf, 1, sizeof(buf));
    if (fread(buf, 1, sizeof(buf), fp) != sizeof(buf)) {
        fprintf(stderr, "read /dev/zero error: %d\n", errno);
        return -1;
    }
    printf("read buf: %x,%x,%x,%x\n", buf[0], buf[1], buf[2], buf[3]);

    memset(buf, 2, sizeof(buf));
    if (fwrite(buf, 1, sizeof(buf), fp) != sizeof(buf)) {
        fprintf(stderr, "write /dev/zero error: %d\n", errno);
        return -1;
    }
    if (fread(buf, 1, sizeof(buf), fp) != sizeof(buf)) {
        fprintf(stderr, "read /dev/zero error: %d\n", errno);
        return -1;
    }
    printf("read after write, buf: %x,%x,%x,%x\n",
           buf[0], buf[1], buf[2], buf[3]);

    fclose(fp);
    printf("Test /dev/zero ok!\n");
    return 0;
}

static int test_null()
{
    FILE *fp;
    char buf[32] = {1};

    printf("Test /dev/null ...\n");
    fp = fopen("/dev/null", "r+");
    if (fp == NULL) {
        fprintf(stderr, "open /dev/null error: %d\n", errno);
        return -1;
    }

    if (fread(buf, 1, sizeof(buf), fp) != 0) {
        fprintf(stderr, "read /dev/null error: %d\n", errno);
        return -1;
    }

    if (fwrite(buf, 1, sizeof(buf), fp) != sizeof(buf)) {
        fprintf(stderr, "write /dev/null error: %d\n", errno);
        return -1;
    }

    fclose(fp);
    printf("Test /dev/null ok!\n");
    return 0;
}

static int test_console()
{
    int fd;
    char buf[] = "hello\n";

    printf("Test /dev/console ...\n");
    fd = open("/dev/console", O_WRONLY);
    if (fd < 0) {
        fprintf(stderr, "open /dev/console error: %s\n", strerror(errno));
        return -1;
    }

    if (write(fd, buf, strlen(buf)+1) != strlen(buf)+1) {
        fprintf(stderr, "write /dev/console error: %s\n", strerror(errno));
        return -1;
    }

    close(fd);
    printf("Test /dev/console ok!\n");
    return 0;
}

int main()
{
    printf("[devfs] ..\n");
    if (test_zero() < 0) {
        fprintf(stderr, "Test /dev/zero error!\n");
    }
    if (test_null() < 0) {
        fprintf(stderr, "Test /dev/null error!\n");
    }
    if (test_console() < 0) {
        fprintf(stderr, "Test /dev/console error!\n");
    }

    return 0;
}
