<div align="center">
  <p align="center"> <img height="50" src="https://github.com/user-attachments/assets/d6d8849e-2c2c-4d8c-b94d-4cf3356fb894" alt="birdtalk"> </p>
  <p align="center">
    <a href="https://www.rust-lang.org/"><img height="30" src="https://img.shields.io/badge/Rust-f75208?style=for-the-badge&logo=rust&logoColor=white"></a>
    <a href="https://dioxuslabs.com/"><img height="30" src="https://img.shields.io/badge/Dioxus-00a8d6?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyBmaWxsPSIjZmZmIiBzdHJva2U9IiNmZmYiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDQ0OCA1MTIiPjwhLS0hRm9udCBBd2Vzb21lIEZyZWUgNi42LjAgYnkgQGZvbnRhd2Vzb21lIC0gaHR0cHM6Ly9mb250YXdlc29tZS5jb20gTGljZW5zZSAtIGh0dHBzOi8vZm9udGF3ZXNvbWUuY29tL2xpY2Vuc2UvZnJlZSBDb3B5cmlnaHQgMjAyNCBGb250aWNvbnMsIEluYy4tLT48cGF0aCBkPSJNNDE2IDBjMTcuNyAwIDMyIDE0LjMgMzIgMzJjMCA1OS44LTMwLjMgMTA3LjUtNjkuNCAxNDYuNmMtMjggMjgtNjIuNSA1My41LTk3LjMgNzcuNGwtMi41IDEuN2MtMTEuOSA4LjEtMjMuOCAxNi4xLTM1LjUgMjMuOWMwIDAgMCAwIDAgMHMwIDAgMCAwczAgMCAwIDBsLTEuNiAxYy02IDQtMTEuOSA3LjktMTcuOCAxMS45Yy0yMC45IDE0LTQwLjggMjcuNy01OS4zIDQxLjVsMTE4LjUgMGMtOS44LTcuNC0yMC4xLTE0LjctMzAuNy0yMi4xbDctNC43IDMtMmMxNS4xLTEwLjEgMzAuOS0yMC42IDQ2LjctMzEuNmMyNSAxOC4xIDQ4LjkgMzcuMyA2OS40IDU3LjdDNDE3LjcgMzcyLjUgNDQ4IDQyMC4yIDQ0OCA0ODBjMCAxNy43LTE0LjMgMzItMzIgMzJzLTMyLTE0LjMtMzItMzJMNjQgNDgwYzAgMTcuNy0xNC4zIDMyLTMyIDMycy0zMi0xNC4zLTMyLTMyYzAtNTkuOCAzMC4zLTEwNy41IDY5LjQtMTQ2LjZjMjgtMjggNjIuNS01My41IDk3LjMtNzcuNGMtMzQuOC0yMy45LTY5LjMtNDkuMy05Ny4zLTc3LjRDMzAuMyAxMzkuNSAwIDkxLjggMCAzMkMwIDE0LjMgMTQuMyAwIDMyIDBTNjQgMTQuMyA2NCAzMmwzMjAgMGMwLTE3LjcgMTQuMy0zMiAzMi0zMnpNMzM4LjYgMzg0bC0yMjkuMiAwYy0xMC4xIDEwLjYtMTguNiAyMS4zLTI1LjUgMzJsMjgwLjIgMGMtNi44LTEwLjctMTUuMy0yMS40LTI1LjUtMzJ6TTEwOS40IDEyOGwyMjkuMiAwYzEwLjEtMTAuNyAxOC42LTIxLjMgMjUuNS0zMkw4My45IDk2YzYuOCAxMC43IDE1LjMgMjEuMyAyNS41IDMyem01NS40IDQ4YzE4LjQgMTMuOCAzOC40IDI3LjUgNTkuMyA0MS41YzIwLjktMTQgNDAuOC0yNy43IDU5LjMtNDEuNWwtMTE4LjUgMHoiLz48L3N2Zz4K"></a>
    <a href="https://supabase.com/"><img height="30" src="https://img.shields.io/badge/Supabase-3ecf8e?style=for-the-badge&logo=supabase&logoColor=white"></a>
    <a href="https://render.com/"><img height="30" src="https://img.shields.io/badge/Render-000?style=for-the-badge&logo=render&logoColor=white"></a>
    <a href="https://tailwindcss.com/"><img height="30" src="https://img.shields.io/badge/Tailwind%20CSS-38b2ac?style=for-the-badge&logo=tailwind-css&logoColor=white"></a>
  </p>
</div>

## development

### dependencies

- [rust](https://rustup.rs/)
- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [supabase cli](https://github.com/supabase/cli/) (and a free account)
- [just](https://github.com/casey/just?tab=readme-ov-file#installation)
- dx: `cargo install dioxus-cli` (or `cargo binstall` if you have it)

### initial setup

- Run `just initial-setup`.
- Log into supabase [locally](https://supabase.com/docs/reference/cli/supabase-login).
- Run `supabase start`.
- `cp .env.example .env` and fill in the anon key found in `supabase status`.

### start local services

- `just supabase-up`: Start up local supabase with some seed data
- `just watch-server`: Start dioxus app running at `localhost:3000` with hot reload
