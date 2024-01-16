const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  await invoke("get_repos_json")
}

document.getElementById("home-button").addEventListener("click", async () => {
  const topContainer = document.querySelector(".top-container .content");

  topContainer.innerHTML = "";

  let repos = JSON.parse(await invoke("get_repos_json"));

  for (const repo of repos) {
    const cheat = document.createElement("div");

    const name = document.createElement("p");

    name.textContent = repo.name;

    cheat.appendChild(name);

    const injectButton = document.createElement("button");

    injectButton.textContent = "Inject";

    injectButton.addEventListener("click", async () => {
      await invoke("run_script", { path: repo.path + "\\" + repo.injectorScript });
    });

    cheat.appendChild(injectButton);

    topContainer.appendChild(cheat);
  }
});