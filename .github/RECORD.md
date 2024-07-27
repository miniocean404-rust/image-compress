## workflow 事件

使用 repository_dispatch 事件:

如果你想从外部系统手动触发 workflow，你可以使用 repository_dispatch 事件。首先，你需要在你的 workflow 文件中配置 repository_dispatch 事件，如下所示：

```yml
name: Repository Dispatch Workflow
on:
repository_dispatch:
types: [manual-trigger]

jobs:
run:
runs-on: ubuntu-latest
steps: - run: echo "This workflow was triggered manually!"
```

然后，你可以使用 GitHub API 发送一个 repository_dispatch 事件到你的仓库，如下所示：

```shell
curl -XPOST -u username:token \
 -H "Accept: application/vnd.github.everest-preview+json" \
 -H "Content-Type: application/json" \
 https://api.github.com/repos/owner/repo/dispatches \
 --data '{"event_type": "manual-trigger"}'
```

在这个命令中，username 和 token 是你的 GitHub 用户名和一个具有适当权限的 personal access token，owner 和 repo 是你的仓库的所有者和名称。
