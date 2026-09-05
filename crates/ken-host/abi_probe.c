#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <stdio.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <unistd.h>

/* Clean-room clearance evt_5fx7gmprrk07b: fixed names and integers only. */
int main(void) {
    printf("POINTER_WIDTH=%lld\n", (long long)(sizeof(void *) * CHAR_BIT));
    printf("POINTER_ALIGNMENT=%lld\n", (long long)_Alignof(void *));
    printf("C_CHAR_WIDTH=%lld\n", (long long)(sizeof(char) * CHAR_BIT));
    printf("C_CHAR_ALIGNMENT=%lld\n", (long long)_Alignof(char));
    printf("C_SHORT_WIDTH=%lld\n", (long long)(sizeof(short) * CHAR_BIT));
    printf("C_SHORT_ALIGNMENT=%lld\n", (long long)_Alignof(short));
    printf("C_INT_WIDTH=%lld\n", (long long)(sizeof(int) * CHAR_BIT));
    printf("C_INT_ALIGNMENT=%lld\n", (long long)_Alignof(int));
    printf("C_LONG_WIDTH=%lld\n", (long long)(sizeof(long) * CHAR_BIT));
    printf("C_LONG_ALIGNMENT=%lld\n", (long long)_Alignof(long));
    printf("C_LONG_LONG_WIDTH=%lld\n", (long long)(sizeof(long long) * CHAR_BIT));
    printf("C_LONG_LONG_ALIGNMENT=%lld\n", (long long)_Alignof(long long));
    printf("C_FLOAT_WIDTH=%lld\n", (long long)(sizeof(float) * CHAR_BIT));
    printf("C_FLOAT_ALIGNMENT=%lld\n", (long long)_Alignof(float));
    printf("C_DOUBLE_WIDTH=%lld\n", (long long)(sizeof(double) * CHAR_BIT));
    printf("C_DOUBLE_ALIGNMENT=%lld\n", (long long)_Alignof(double));
    printf("O_RDONLY=%lld\n", (long long)O_RDONLY);
    printf("O_WRONLY=%lld\n", (long long)O_WRONLY);
    printf("O_RDWR=%lld\n", (long long)O_RDWR);
    printf("O_APPEND=%lld\n", (long long)O_APPEND);
    printf("O_CREAT=%lld\n", (long long)O_CREAT);
    printf("O_EXCL=%lld\n", (long long)O_EXCL);
    printf("O_TRUNC=%lld\n", (long long)O_TRUNC);
    printf("O_DIRECTORY=%lld\n", (long long)O_DIRECTORY);
    printf("O_NOFOLLOW=%lld\n", (long long)O_NOFOLLOW);
    printf("O_CLOEXEC=%lld\n", (long long)O_CLOEXEC);
    printf("AT_REMOVEDIR=%lld\n", (long long)AT_REMOVEDIR);
    printf("MODE_FILE_CREATE=%lld\n", (long long)(S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH));
    printf("MODE_DIRECTORY_CREATE=%lld\n", (long long)(S_IRWXU | S_IRWXG | S_IRWXO));
    printf("SYS_OPENAT=%lld\n", (long long)SYS_openat);
    printf("SYS_MKDIRAT=%lld\n", (long long)SYS_mkdirat);
    printf("SYS_UNLINKAT=%lld\n", (long long)SYS_unlinkat);
    printf("SYS_RENAMEAT=%lld\n", (long long)SYS_renameat);
    printf("SYS_READLINKAT=%lld\n", (long long)SYS_readlinkat);
    printf("SYS_FCHMOD=%lld\n", (long long)SYS_fchmod);
    printf("ERRNO_ENOENT=%lld\n", (long long)ENOENT);
    printf("ERRNO_EEXIST=%lld\n", (long long)EEXIST);
    return 0;
}
