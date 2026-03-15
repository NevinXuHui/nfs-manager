import { createSignal, onMount, For, Show } from "solid-js";
import { getConfigs, getStatus, mountNfs, umountNfs, mountAll, umountAll, addConfig, removeConfig, openMountPoint, type NfsConfig, type MountStatus } from "./api";
import "./App.css";

function App() {
  const [configs, setConfigs] = createSignal<NfsConfig[]>([]);
  const [statuses, setStatuses] = createSignal<MountStatus[]>([]);
  const [showAddDialog, setShowAddDialog] = createSignal(false);
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal("");

  const [newConfig, setNewConfig] = createSignal<NfsConfig>({
    name: "",
    server: "",
    mount_point: "",
    options: "rw,sync,hard,intr",
  });

  async function loadData() {
    try {
      const [cfgs, sts] = await Promise.all([getConfigs(), getStatus()]);
      setConfigs(cfgs);
      setStatuses(sts);
      setError("");
    } catch (e) {
      setError(String(e));
    }
  }

  onMount(() => {
    loadData();
  });

  async function handleMount(name: string) {
    setLoading(true);
    try {
      await mountNfs(name);
      await loadData();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleUmount(name: string) {
    setLoading(true);
    try {
      await umountNfs(name, false);
      await loadData();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleMountAll() {
    setLoading(true);
    try {
      const errors = await mountAll();
      if (errors.length > 0) {
        setError(errors.join("\n"));
      }
      await loadData();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleUmountAll() {
    setLoading(true);
    try {
      const errors = await umountAll(false);
      if (errors.length > 0) {
        setError(errors.join("\n"));
      }
      await loadData();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleAddConfig() {
    const cfg = newConfig();
    if (!cfg.name || !cfg.server || !cfg.mount_point) {
      setError("请填写所有必填字段");
      return;
    }

    setLoading(true);
    try {
      await addConfig(cfg);
      setShowAddDialog(false);
      setNewConfig({
        name: "",
        server: "",
        mount_point: "",
        options: "rw,sync,hard,intr",
      });
      await loadData();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleRemove(name: string) {
    if (!confirm(`确定要删除配置 "${name}" 吗？`)) {
      return;
    }

    setLoading(true);
    try {
      await removeConfig(name);
      await loadData();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleOpen(name: string) {
    setLoading(true);
    try {
      await openMountPoint(name);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  function getStatusForConfig(name: string): MountStatus | undefined {
    return statuses().find(s => s.name === name);
  }

  return (
    <main class="container">
      <header>
        <h1>NFS Manager</h1>
        <button onClick={() => setShowAddDialog(true)} disabled={loading()}>
          + 添加配置
        </button>
      </header>

      <Show when={error()}>
        <div class="error" onClick={() => setError("")}>
          {error()}
        </div>
      </Show>

      <div class="config-list">
        <For each={configs()}>
          {(config) => {
            const status = () => getStatusForConfig(config.name);
            const isMounted = () => status()?.mounted ?? false;

            return (
              <div class="config-card">
                <div class="config-header">
                  <span class={`status-dot ${isMounted() ? "mounted" : "unmounted"}`} />
                  <h3>{config.name}</h3>
                  <span class="status-text">{isMounted() ? "已挂载" : "未挂载"}</span>
                </div>
                <div class="config-info">
                  <p><strong>服务器:</strong> {config.server}</p>
                  <p><strong>挂载点:</strong> {config.mount_point}</p>
                  <p><strong>选项:</strong> {config.options}</p>
                </div>
                <div class="config-actions">
                  <Show when={isMounted()} fallback={
                    <button onClick={() => handleMount(config.name)} disabled={loading()}>
                      挂载
                    </button>
                  }>
                    <button onClick={() => handleUmount(config.name)} disabled={loading()}>
                      卸载
                    </button>
                    <button onClick={() => handleOpen(config.name)} disabled={loading()} class="secondary">
                      打开
                    </button>
                  </Show>
                  <button onClick={() => handleRemove(config.name)} disabled={loading()} class="danger">
                    删除
                  </button>
                </div>
              </div>
            );
          }}
        </For>
      </div>

      <footer>
        <button onClick={handleMountAll} disabled={loading()}>
          挂载全部
        </button>
        <button onClick={handleUmountAll} disabled={loading()}>
          卸载全部
        </button>
      </footer>

      <Show when={showAddDialog()}>
        <div class="dialog-overlay" onClick={() => setShowAddDialog(false)}>
          <div class="dialog" onClick={(e) => e.stopPropagation()}>
            <h2>添加 NFS 配置</h2>
            <form onSubmit={(e) => { e.preventDefault(); handleAddConfig(); }}>
              <label>
                名称 *
                <input
                  type="text"
                  value={newConfig().name}
                  onInput={(e) => setNewConfig({ ...newConfig(), name: e.currentTarget.value })}
                  placeholder="例如: company"
                  required
                />
              </label>
              <label>
                服务器地址 *
                <input
                  type="text"
                  value={newConfig().server}
                  onInput={(e) => setNewConfig({ ...newConfig(), server: e.currentTarget.value })}
                  placeholder="例如: 192.168.1.100:/data"
                  required
                />
              </label>
              <label>
                挂载点 *
                <input
                  type="text"
                  value={newConfig().mount_point}
                  onInput={(e) => setNewConfig({ ...newConfig(), mount_point: e.currentTarget.value })}
                  placeholder="例如: company-data"
                  required
                />
              </label>
              <label>
                挂载选项
                <input
                  type="text"
                  value={newConfig().options}
                  onInput={(e) => setNewConfig({ ...newConfig(), options: e.currentTarget.value })}
                  placeholder="例如: rw,sync,hard,intr"
                />
              </label>
              <div class="dialog-actions">
                <button type="button" onClick={() => setShowAddDialog(false)}>
                  取消
                </button>
                <button type="submit" disabled={loading()}>
                  添加
                </button>
              </div>
            </form>
          </div>
        </div>
      </Show>
    </main>
  );
}

export default App;
