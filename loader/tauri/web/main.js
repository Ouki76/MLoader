const { invoke } = window.__TAURI__.tauri;

async function clearPage() {
  const topContainer = document.querySelector(".top-container .content");

  topContainer.innerHTML = "";

  const updateAllButton = document.getElementById("update-all-button");

  if (updateAllButton != null) {
    updateAllButton.remove();
  }

  return topContainer;
}

document.getElementById("home-button").addEventListener("click", async () => {
  const topContainer = await clearPage();

  let repos = JSON.parse(await invoke("get_cheats_json"));

  for (const repo of repos) {
    const cheat = document.createElement("div");

    cheat.className = "cheat";

    const name = document.createElement("p");

    name.textContent = repo.path.split("\\").reverse()[0];

    cheat.appendChild(name);

    cheat.addEventListener("click", async () => {
      const cheatPage = document.createElement("div");

      cheatPage.style = `
        z-index: 999; 
        position: absolute; 

        top: 15px; 
        left: 15px; 
        right: 15px; 
        bottom: 15px; 

        width: 100 %; 
        height: 100 %; 

        border: 1px solid white;
        border-radius: var(--border-radius);
        background-color: black;
      `;

      const barContainer = document.createElement("div");

      const title = document.createElement("p");

      title.style = "font-size: 14px; margin: 0; padding: 0;";

      title.textContent = repo.path.split("\\").reverse()[0];

      barContainer.appendChild(title);

      barContainer.style = `
        position: relative;

        display: flex; 
        justify-content: space-between; 

        background-color: brown;

        border-radius: var(--border-radius) var(--border-radius) 0 0;

        padding: 5px;
      `;

      const closeButton = document.createElement("button");

      closeButton.style = `
        float: right;

        width: 20px;
        height: 20px;

        background-color: red;
        border-radius: 50%;

        border: none;
      `;

      closeButton.addEventListener("click", () => {
        cheatPage.remove();
      });

      barContainer.appendChild(closeButton);

      cheatPage.appendChild(barContainer);

      await invoke("get_file_content", { path: repo.path + "\\" + repo.pageScript }).then((data) => {
        eval(data);
      });

      document.body.appendChild(cheatPage);
    });

    topContainer.appendChild(cheat);
  }
});

document.getElementById("repos-button").addEventListener("click", async () => {
  const topContainer = await clearPage();

  const bottomRightContainer = document.querySelector(".bottom-container .right");

  const updateAllButton = document.createElement("button");

  updateAllButton.id = "update-all-button";

  const updateAllIcon = document.createElement("img");

  updateAllIcon.src = "../assets/images/update-svgrepo-com.svg";

  updateAllButton.appendChild(updateAllIcon);

  updateAllButton.addEventListener("click", async () => {
    await invoke("update_all_repos");
  });

  bottomRightContainer.appendChild(updateAllButton);

  let repos = await invoke("get_all_repos");

  for (const repo of repos) {
    const cheat = document.createElement("div");

    cheat.className = "cheat";

    const name = document.createElement("p");

    name.textContent = repo.split("\\").reverse()[0];

    cheat.appendChild(name);

    topContainer.appendChild(cheat);
  }
});

document.getElementById("notifications-button").addEventListener("click", async () => {
  const topContainer = await clearPage();
});