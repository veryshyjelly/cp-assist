# CP-assist

## App preview
![alt text](https://github.com/veryshyjelly/cp-assist/blob/main/cp-assist-shot.png?raw=true)

![alt text](https://github.com/veryshyjelly/cp-assist/blob/main/cp-assist-cast.webm?raw=true)

## Installation
- Prerequisites:
    - competitive-companion [link](https://github.com/jmerle/competitive-companion)
    - judge0 installation (local or remote)
        1. install docker and docker-compose
        2. Download and extract the release archive:
        ```
            wget https://github.com/veryshyjelly/judge0/releases/download/v69/judge0.zip
            unzip judge0.zip
            cd judge0
        ```
        3. Add password for `POSTGRES_PASSWORD` and `REDIS_PASSWORD` in `judge0.conf` file
        4. Run all services and wait a few seconds until everything is initialized:
        ```
            docker-compose up -d db redis
            docker-compose up -d
        ```
        
- Linux:
    - arch linux:
    ```
        wget https://github.com/veryshyjelly/cp-assist/releases/download/v0.1.1-alpha/cp-assist-0.1.1-1-x86_64.pkg.tar.zst
        pacman -U cp-assist-0.1.1-1-x86_64.pkg.tar.zst
    ```
    - debian: [.deb file](https://github.com/veryshyjelly/cp-assist/releases/download/v0.1.1-alpha/cp-assist_0.1.1_amd64.deb)
    - red-hat: [.rpm file](https://github.com/veryshyjelly/cp-assist/releases/download/v0.1.1-alpha/cp-assist-0.1.1-1.x86_64.rpm)

- Windows:
    - msi installer: [msi](https://github.com/veryshyjelly/cp-assist/releases/download/v0.1.1-alpha/cp-assist_0.1.1_x64_en-US.msi)
    - setup exe: [exe](https://github.com/veryshyjelly/cp-assist/releases/download/v0.1.1-alpha/cp-assist_0.1.1_x64-setup.exe)