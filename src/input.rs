/// Input de texto de una sola línea.
/// Se guarda como `Vec<char>` para poder insertar/borrar caracteres en
/// cualquier posición sin re-correr la cadena.
pub struct Input {
    /// Posición del cursor (índice donde se insertará el próximo carácter).
    pub position: usize,
    /// Caracteres del texto ingresado.
    pub content: Vec<char>,
}

impl Input {
    // ── Edición de contenido ───────────────────────────────────────────

    /// Vacía el input y vuelve el cursor al inicio.
    pub fn clear(&mut self) {
        self.position = 0;
        self.content.clear();
    }

    /// Inserta `ch` en la posición actual del cursor y avanza el cursor.
    /// Ignora saltos de línea (`'\n'`), ya que este input es de una sola línea.
    pub fn set_char(&mut self, ch: char) {
        if ch != '\n' {
            self.content.insert(self.position, ch);
            self.position += 1
        }
    }

    /// Inserta una cadena completa, carácter por carácter, en la posición
    /// actual del cursor (por ejemplo al pegar texto con paste).
    pub fn set_string(&mut self, s: String) {
        s.chars().for_each(|ch| self.set_char(ch));
    }

    /// Borra el carácter justo antes del cursor (comportamiento de
    /// "Backspace") y retrocede el cursor una posición.
    pub fn remove_char(&mut self) {
        if self.position > 0 {
            self.content.remove(self.position - 1);
            self.position -= 1
        }
    }

    // ── Movimiento del cursor ──────────────────────────────────────────

    /// Mueve el cursor una posición a la izquierda, sin pasar del inicio.
    pub fn move_cursor_left(&mut self) {
        if self.position > 0 {
            self.position -= 1
        }
    }

    /// Mueve el cursor una posición a la derecha, sin pasar del final.
    pub fn move_cursor_right(&mut self) {
        if self.position < self.content.len() {
            self.position += 1
        }
    }

    // ── Conversión ─────────────────────────────────────────────────────

    /// Junta el contenido (`Vec<char>`) en un `String`.
    pub fn to_string(&self) -> String {
        self.content.iter().collect()
    }
}

impl Default for Input {
    /// Input vacío, con el cursor en la posición 0.
    fn default() -> Self {
        return Input {
            position: 0,
            content: Vec::new(),
        };
    }
}
