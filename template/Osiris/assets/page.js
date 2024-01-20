let injectButton = document.createElement("button");

injectButton.textContent = "Inject";

injectButton.addEventListener("click", async () => {
    await invoke("run_script", { path: repo.path + "\\" + repo.injectorScript });
})

injectButton.style = `
    color: white;
    background-color: black;

    border: 1px solid white;

    border-radius: var(--border-radius);
`;

cheatPage.appendChild(injectButton);