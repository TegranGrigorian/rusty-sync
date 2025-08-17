<!-- This file will contain docuemntation on the json format -->

# JSON File Format
In order to avoid merge conflicts we need to differniate between a git repo and not a git repo

## Format
### git repo
```shell
{
  "name": "Projects",
  "type": "folder",
  "children": [
    {
      "name": "rusty-sync",
      "type": "folder",
      "children": null,
      "git_remote": "https://github.com/TegranGrigorian/rusty-sync"
    }
  ],
  "git_remote": null
}
```

---

### not-git repo
```shell
{
  "name": "Downloads",
  "type": "folder",
  "children": [
    {
      "name": "EXAMPLE.struct_git.json",
      "type": "file",
      "children": null,
      "git_remote": null
    },
    {
      "name": "rat_x86_03082025.deb",
      "type": "file",
      "children": null,
      "git_remote": null
    },
    {
      "name": "yt-to-mp3-4-linux-x86",
      "type": "folder",
      "children": [
        {
          "name": "bin",
          "type": "folder",
          "children": [
            {
              "name": "linux",
              "type": "folder",
              "children": [
                {
                  "name": "ffprobe",
                  "type": "file",
                  "children": null,
                  "git_remote": null
                },
                {
                  "name": "install.sh",
                  "type": "file",
                  "children": null,
                  "git_remote": null
                },
                {
                  "name": "ffmpeg",
                  "type": "file",
                  "children": null,
                  "git_remote": null
                },
                {
                  "name": "yt-dlp",
                  "type": "file",
                  "children": null,
                  "git_remote": null
                },
                {
                  "name": "yt-to-mp3-4",
                  "type": "file",
                  "children": null,
                  "git_remote": null
                },
                {
                  "name": "bin",
                  "type": "folder",
                  "children": [
                    {
                      "name": "linux",
                      "type": "folder",
                      "children": [
                        {
                          "name": "ffprobe",
                          "type": "file",
                          "children": null,
                          "git_remote": null
                        },
                        {
                          "name": "ffmpeg",
                          "type": "file",
                          "children": null,
                          "git_remote": null
                        },
                        {
                          "name": "yt-dlp",
                          "type": "file",
                          "children": null,
                          "git_remote": null
                        }
                      ],
                      "git_remote": null
                    }
                  ],
                  "git_remote": null
                }
              ],
              "git_remote": null
            }
          ],
          "git_remote": null
        }
      ],
      "git_remote": null
    },
    {
      "name": "yt-to-mp3-4-linux-x86.tar.gz",
      "type": "file",
      "children": null,
      "git_remote": null
    }
  ],
  "git_remote": null
}
```

---

## Inspiriation:
The format is inspiration from tree command
```shell
Downloads/
├── EXAMPLE.struct_git.json
├── rat_x86_03082025.deb
├── yt-to-mp3-4-linux-x86
│   └── bin
│       └── linux
│           ├── bin
│           │   └── linux
│           │       ├── ffmpeg
│           │       ├── ffprobe
│           │       └── yt-dlp
│           ├── ffmpeg
│           ├── ffprobe
│           ├── install.sh
│           ├── yt-dlp
│           └── yt-to-mp3-4
└── yt-to-mp3-4-linux-x86.tar.gz
```