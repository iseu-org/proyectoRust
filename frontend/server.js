const express = require("express");
const path = require("path");
const app = express();

app.use(express.static(path.join(__dirname, "public")));

// Node ahora actúa como un Proxy hacia el servidor de Rust
app.get("/usuario", async (req, res) => {
    const token = req.query.token;

    if (!token) {
        return res.status(400).json({ error: "El token es requerido" });
    }

    try {
        // LLAMADA A RUST
        const rustResponse = await fetch(`http://127.0.0.1:3000/github?token=${token}`);

        if (!rustResponse.ok) {
            // Si Rust nos da error (ej. 401), se lo pasamos al frontend
            const errorData = await rustResponse.text();
            return res.status(rustResponse.status).json({ error: errorData });
        }

        const user = await rustResponse.json();

        // Node recibe la estructura GitHubUser de Rust y la envía al frontend
        res.json(user);

    } catch (err) {
        console.error("Error conectando con Rust:", err);
        res.status(500).json({ error: "No se pudo conectar con el servicio de Rust" });
    }
});

const PORT = 8080;
app.listen(PORT, () => {
    console.log(`🌐 Frontend & Node Gateway en http://localhost:${PORT}`);
});