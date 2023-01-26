#ifdef _WIN32

#include <windows.h>
int getcols() {
    CONSOLE_SCREEN_BUFFER_INFO csbi;
    if (GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &csbi) == 0) {
        return -1;
    }
    return csbi.srWindow.Right - csbi.srWindow.Left + 1;
}
int getlines() {
    CONSOLE_SCREEN_BUFFER_INFO csbi;
    if (GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &csbi) == 0) {
        return -1;
    }
    return csbi.srWindow.Bottom - csbi.srWindow.Top + 1;
}

#else

#include <sys/ioctl.h>
#include <stdio.h>
#include <unistd.h>
int getcols() {
    struct winsize w;

    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) == -1) {
        return -1;
    }
    return w.ws_col;
}
int getlines() {
    struct winsize w;
    
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) == -1) {
        return -1;
    }
    return w.ws_row;
}

#endif