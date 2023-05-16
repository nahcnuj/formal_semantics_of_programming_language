｢プログラミング言語の形式的意味論入門」（丸善出版）に登場するプログラミング言語の、意味論の Rust による実装

* src/imp.rs : $`\mathbf{IMP}`$

## 形式検証

TODO

参考：[Rustの形式検証ツールCreusotを触ってみる](https://zenn.dev/kk/articles/20230213_creusot_intro)

## 環境構築メモ

### WSL 2 + Docker で GUI アプリ

[DockerからGUIを使ってみよう \| インフォメーション・ディベロプメント](https://www.idnet.co.jp/column/page_229.html)

### WSL 2 の Ubuntu 22.04 で root-less に Docker を動かす

* [DockerをRootlessモードで利用する](https://zenn.dev/nemolize/articles/3cec197e5f5ec8)
* [Run the Docker daemon as a non\-root user \(Rootless mode\) \| Docker Documentation](https://docs.docker.com/engine/security/rootless/)

Docker デーモンを停止

```sh
sudo service docker stop
```

> You must install `newuidmap` and `newgidmap` on the host. These commands are provided by the `uidmap` package on most distros.

```console
$ sudo apt-get install -y uidmap
$ which newuidmap
/usr/bin/newuidmap
$ which newgidmap
/usr/bin/newgidmap
```

> Install `dbus-user-session` package if not installed. Run `sudo apt-get install -y dbus-user-session` and relogin.

```sh
sudo apt-get install -y dbus-user-session
```

> Run `dockerd-rootless-setuptool.sh install` as a non-root user to set up the daemon:

```console
$ ls /usr/bin/dockerd*
/usr/bin/dockerd  /usr/bin/dockerd-rootless-setuptool.sh  /usr/bin/dockerd-rootless.sh
$ dockerd-rootless-setuptool.sh install --force --skip-iptables
[INFO] Creating /home/user/.config/systemd/user/docker.service
[INFO] starting systemd service docker.service
+ systemctl --user start docker.service
+ sleep 3
+ systemctl --user --no-pager --full status docker.service
● docker.service - Docker Application Container Engine (Rootless)
     Loaded: loaded (/home/user/.config/systemd/user/docker.service; disabled; vendor preset: enabled)
     Active: active (running) since Tue 2023-05-16 01:04:17 JST; 3s ago
       Docs: https://docs.docker.com/go/rootless/
   Main PID: 1108 (rootlesskit)
     CGroup: /user.slice/user-1000.slice/user@1000.service/app.slice/docker.service
             ├─1108 rootlesskit --net=slirp4netns --mtu=65520 --slirp4netns-sandbox=auto --slirp4netns-seccomp=auto --disable-host-loopback --port-driver=builtin --copy-up=/etc --copy-up=/run --propagation=rslave /usr/bin/dockerd-rootless.sh --iptables=false
             ├─1119 /proc/self/exe --net=slirp4netns --mtu=65520 --slirp4netns-sandbox=auto --slirp4netns-seccomp=auto --disable-host-loopback --port-driver=builtin --copy-up=/etc --copy-up=/run --propagation=rslave /usr/bin/dockerd-rootless.sh --iptables=false
             ├─1137 slirp4netns --mtu 65520 -r 3 --disable-host-loopback --enable-sandbox --enable-seccomp 1119 tap0
             ├─1144 dockerd --iptables=false
             └─1174 containerd --config /run/user/1000/docker/containerd/containerd.toml --log-level info

May 16 01:04:17 LAPTOP-ABIKGU9I dockerd-rootless.sh[1144]: time="2023-05-16T01:04:17.277804620+09:00" level=info msg="[graphdriver] using prior storage driver: overlay2"
May 16 01:04:17 LAPTOP-ABIKGU9I dockerd-rootless.sh[1144]: time="2023-05-16T01:04:17.284830521+09:00" level=info msg="Loading containers: start."
May 16 01:04:17 LAPTOP-ABIKGU9I dockerd-rootless.sh[1144]: time="2023-05-16T01:04:17.298416947+09:00" level=info msg="Default bridge (docker0) is assigned with an IP address 172.17.0.0/16. Daemon option --bip can be used to set a preferred IP address"
May 16 01:04:17 LAPTOP-ABIKGU9I dockerd-rootless.sh[1144]: time="2023-05-16T01:04:17.304162052+09:00" level=info msg="Loading containers: done."
May 16 01:04:17 LAPTOP-ABIKGU9I dockerd-rootless.sh[1144]: time="2023-05-16T01:04:17.326049553+09:00" level=warning msg="Not using native diff for overlay2, this may cause degraded performance for building images: running in a user namespace" storage-driver=overlay2
May 16 01:04:17 LAPTOP-ABIKGU9I dockerd-rootless.sh[1144]: time="2023-05-16T01:04:17.326169772+09:00" level=warning msg="WARNING: Running in rootless-mode without cgroups. To enable cgroups in rootless-mode, you need to boot the system in cgroup v2 mode."
May 16 01:04:17 LAPTOP-ABIKGU9I dockerd-rootless.sh[1144]: time="2023-05-16T01:04:17.326185011+09:00" level=info msg="Docker daemon" commit=9dbdbd4 graphdriver=overlay2 version=23.0.6
May 16 01:04:17 LAPTOP-ABIKGU9I dockerd-rootless.sh[1144]: time="2023-05-16T01:04:17.326482947+09:00" level=info msg="Daemon has completed initialization"
May 16 01:04:17 LAPTOP-ABIKGU9I systemd[328]: Started Docker Application Container Engine (Rootless).
May 16 01:04:17 LAPTOP-ABIKGU9I dockerd-rootless.sh[1144]: time="2023-05-16T01:04:17.342590319+09:00" level=info msg="API listen on /run/user/1000/docker.sock"
+ DOCKER_HOST=unix:///run/user/1000//docker.sock /usr/bin/docker version
Client: Docker Engine - Community
 Version:           23.0.6
 API version:       1.42
 Go version:        go1.19.9
 Git commit:        ef23cbc
 Built:             Fri May  5 21:18:13 2023
 OS/Arch:           linux/amd64
 Context:           default

Server: Docker Engine - Community
 Engine:
  Version:          23.0.6
  API version:      1.42 (minimum version 1.12)
  Go version:       go1.19.9
  Git commit:       9dbdbd4
  Built:            Fri May  5 21:18:13 2023
  OS/Arch:          linux/amd64
  Experimental:     false
 containerd:
  Version:          1.6.21
  GitCommit:        3dce8eb055cbb6872793272b4f20ed16117344f8
 runc:
  Version:          1.1.7
  GitCommit:        v1.1.7-0-g860f061
 docker-init:
  Version:          0.19.0
  GitCommit:        de40ad0
 rootlesskit:
  Version:          1.1.0
  ApiVersion:       1.1.1
  NetworkDriver:    slirp4netns
  PortDriver:       builtin
  StateDir:         /tmp/rootlesskit2120473135
 slirp4netns:
  Version:          1.0.1
  GitCommit:        6a7b16babc95b6a3056b33fb45b74a6f62262dd4
+ systemctl --user enable docker.service
Created symlink /home/user/.config/systemd/user/default.target.wants/docker.service → /home/user/.config/systemd/user/docker.service.
[INFO] Installed docker.service successfully.
[INFO] To control docker.service, run: `systemctl --user (start|stop|restart) docker.service`
[INFO] To run docker.service on system startup, run: `sudo loginctl enable-linger user`

[INFO] Creating CLI context "rootless"
Successfully created context "rootless"
[INFO] Using CLI context "rootless"
Current context is now "rootless"

[INFO] Make sure the following environment variable(s) are set (or add them to ~/.bashrc):
export PATH=/usr/bin:$PATH

[INFO] Some applications may require the following environment variable too:
export DOCKER_HOST=unix:///run/user/1000//docker.sock
```

test

```console
$ docker run --rm -d -p 8080:80 nginx
Unable to find image 'nginx:latest' locally
docker: Error response from daemon: Get "https://registry-1.docker.io/v2/": dial tcp: lookup registry-1.docker.io on 10.0.2.3:53: read udp 10.0.2.100:37277->10.0.2.3:53: i/o timeout.
See 'docker run --help'.
```

[WSL2 環境で docker pull に失敗した時の対処方法 \| メモ](https://al-batross.net/2020/10/02/wsl2-howtopulldockerimagewithouttimeouterror/)

```console
$ tail -2 /etc/wsl.conf
[network]
generateResolvConf = false
$ cat /etc/resolv.conf
cat: /etc/resolv.conf: No such file or directory
$ echo nameserver 8.8.8.8 | sudo tee -a /etc/resolv.conf
```

```console
$ docker run --rm -d -p 8080:80 nginx
Unable to find image 'nginx:latest' locally
latest: Pulling from library/nginx
9e3ea8720c6d: Pull complete 
bf36b6466679: Pull complete 
15a97cf85bb8: Pull complete 
9c2d6be5a61d: Pull complete 
6b7e4a5c7c7a: Pull complete 
8db4caa19df8: Pull complete 
Digest: sha256:480868e8c8c797794257e2abd88d0f9a8809b2fe956cbfbc05dcc0bca1f7cd43
Status: Downloaded newer image for nginx:latest
9d7cbe18fa4dd880b662538b5554df73b8f81931ed2c15fb19b87aa10daa3908
```

http://localhost:8080 OK

### Why3

[2\. Getting Started — Why3 1\.6\.0 documentation](https://why3.lri.fr/doc/starting.html)
の例

```console
$ cat hello.why
theory HelloProof

  goal G1: true

  goal G2: (false -> false) /\ (true \/ false)

  use int.Int

  goal G3: forall x:int. x * x >= 0

end
$ docker compose run --rm why3 ide hello.why
$ docker compose run --rm why3 replay hello
 3/3 (replay OK)
$
```

### Creusot
