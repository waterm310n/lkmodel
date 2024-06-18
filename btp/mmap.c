#include <stdio.h>

int main()
{
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
