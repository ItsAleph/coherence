To: perl5-porters@perl.org
From: fuckyou@suka
Subject: Jfijs, dknskfnnr eiebfjjbbsjwj. Msbfj djwijfn sjsbf ddjd dks?
Reply-To: fuckyou@suka
Message-Id: <5.36.1_27272_1684490622@localhost.localdomain>
Cc: builduser


This is a bug report for perl from fuckyou@suka,
generated with the help of perlbug 1.42 running under perl 5.36.1.


-----------------------------------------------------------------
[Please describe your issue here]



[Please do not change anything below this line]
-----------------------------------------------------------------
---
Flags:
    category=core
    severity=critical
---
Site configuration information for perl 5.36.1:

Configured by builduser at Sun May  7 02:14:56 PM UTC 2023.

Summary of my perl5 (revision 5 version 36 subversion 1) configuration:
   
  Platform:
    osname=linux
    osvers=5.12.15-arch1-1
    archname=aarch64-linux-thread-multi
    uname='archlinux'
    config_args='-des -Dusethreads -Duseshrplib -Doptimize=-march=armv8-a -O2 -pipe -fstack-protector-strong -fno-plt -fexceptions	   -Wp,-D_FORTIFY_SOURCE=2 -Wformat -Werror=format-security	    -fstack-clash-protection -Dprefix=/usr -Dvendorprefix=/usr -Dprivlib=/usr/share/perl5/core_perl -Darchlib=/usr/lib/perl5/5.36/core_perl -Dsitelib=/usr/share/perl5/site_perl -Dsitearch=/usr/lib/perl5/5.36/site_perl -Dvendorlib=/usr/share/perl5/vendor_perl -Dvendorarch=/usr/lib/perl5/5.36/vendor_perl -Dscriptdir=/usr/bin/core_perl -Dsitescript=/usr/bin/site_perl -Dvendorscript=/usr/bin/vendor_perl -Dinc_version_list=none -Dman1ext=1perl -Dman3ext=3perl -Dlddlflags=-shared -Wl,-O1,--sort-common,--as-needed,-z,relro,-z,now -Dldflags=-Wl,-O1,--sort-common,--as-needed,-z,relro,-z,now'
    hint=recommended
    useposix=true
    d_sigaction=define
    useithreads=define
    usemultiplicity=define
    use64bitint=define
    use64bitall=define
    uselongdouble=undef
    usemymalloc=n
    default_inc_excludes_dot=define
  Compiler:
    cc='cc'
    ccflags ='-D_REENTRANT -D_GNU_SOURCE -fwrapv -fno-strict-aliasing -pipe -fstack-protector-strong -I/usr/local/include -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64'
    optimize='-march=armv8-a -O2 -pipe -fstack-protector-strong -fno-plt -fexceptions -Wp,-D_FORTIFY_SOURCE=2 -Wformat -Werror=format-security -fstack-clash-protection'
    cppflags='-D_REENTRANT -D_GNU_SOURCE -fwrapv -fno-strict-aliasing -pipe -fstack-protector-strong -I/usr/local/include'
    ccversion=''
    gccversion='12.1.0'
    gccosandvers=''
    intsize=4
    longsize=8
    ptrsize=8
    doublesize=8
    byteorder=12345678
    doublekind=3
    d_longlong=define
    longlongsize=8
    d_longdbl=define
    longdblsize=16
    longdblkind=1
    ivtype='long'
    ivsize=8
    nvtype='double'
    nvsize=8
    Off_t='off_t'
    lseeksize=8
    alignbytes=8
    prototype=define
  Linker and Libraries:
    ld='cc'
    ldflags ='-Wl,-O1,--sort-common,--as-needed,-z,relro,-z,now -fstack-protector-strong -L/usr/local/lib'
    libpth=/usr/local/lib /usr/lib
    libs=-lpthread -lgdbm -ldb -ldl -lm -lcrypt -lutil -lc -lgdbm_compat
    perllibs=-lpthread -ldl -lm -lcrypt -lutil -lc
    libc=/lib/../lib/libc.so.6
    so=so
    useshrplib=true
    libperl=libperl.so
    gnulibc_version='2.35'
  Dynamic Linking:
    dlsrc=dl_dlopen.xs
    dlext=so
    d_dlsymun=undef
    ccdlflags='-Wl,-E -Wl,-rpath,/usr/lib/perl5/5.36/core_perl/CORE'
    cccdlflags='-fPIC'
    lddlflags='-shared -Wl,-O1,--sort-common,--as-needed,-z,relro,-z,now -L/usr/local/lib -fstack-protector-strong'


---
@INC for perl 5.36.1:
    /usr/lib/perl5/5.36/site_perl
    /usr/share/perl5/site_perl
    /usr/lib/perl5/5.36/vendor_perl
    /usr/share/perl5/vendor_perl
    /usr/lib/perl5/5.36/core_perl
    /usr/share/perl5/core_perl

---
Environment for perl 5.36.1:
    HOME=/home/lena
    LANG=C
    LANGUAGE (unset)
    LD_LIBRARY_PATH (unset)
    LOGDIR (unset)
    PATH=/home/lena/.scripts:/home/lena/.local/bin:/home/lena/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl
    PERL_BADLANG (unset)
    SHELL=/bin/fish
