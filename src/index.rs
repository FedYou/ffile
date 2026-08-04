/// Índice de selección de una lista (ej. el FilePanel o el Sidebar).
/// Guía qué elemento está resaltado y maneja la navegación circular.
pub struct SelectionIndex {
    /// Posición actualmente seleccionada dentro de la lista.
    pub selected: usize,
    /// Cantidad total de elementos de la lista.
    pub count: usize,
    /// `false` cuando el panel no tiene ninguna selección activa (ej. lista
    /// vacía). Mientras sea `false`, `next`/`prev` no hacen nada.
    pub valid: bool,
    /// Índices que se saltan al navegar con `next`/`prev` (ej. separadores).
    pub excluded: Vec<usize>,
}

impl SelectionIndex {
    // ── Inicialización ────────────────────────────────────────────────

    /// Configura el índice para una lista de `count` elementos, dejándolo
    /// válido. `selected` y `excluded` son opcionales: si no se pasan, se
    /// conserva el valor que ya tenía.
    pub fn setup(&mut self, count: usize, selected: Option<usize>, excluded: Option<Vec<usize>>) {
        self.valid = true;
        self.count = count;
        if let Some(s) = selected {
            self.selected = s;
        }
        if let Some(i) = excluded {
            self.excluded = i;
        }
    }

    /// Vuelve el índice a su estado inicial (sin selección válida, sin
    /// elementos, sin exclusiones). Se usa cuando la lista queda vacía.
    pub fn reset(&mut self) {
        self.valid = false;
        self.count = 0;
        self.selected = 0;
        self.excluded.clear();
    }

    /// Fija la selección en un índice puntual, sin pasar por `next`/`prev`.
    pub fn set_selected(&mut self, selected: usize) {
        self.selected = selected;
    }

    // ── Navegación ──────────────────────────────────────────────────────

    /// Avanza la selección una posición, saltando índices `excluded` y
    /// dando la vuelta al llegar al final (circular). Si el índice no es
    /// válido, no hace nada.
    pub fn next(&mut self) {
        if !self.valid {
            return;
        }
        let mut selected = self.selected;

        // Avanza en todas las posiciones, hasta encontrar una posición
        // válida que no esté excluida
        for _ in 0..self.count {
            selected += 1;
            if selected == self.count {
                selected = 0;
            }
            if !self.excluded.contains(&selected) {
                break;
            }
        }

        self.selected = selected
    }

    /// Retrocede la selección una posición, saltando índices `excluded` y
    /// dando la vuelta al llegar al principio (circular). Si el índice no es
    /// válido, no hace nada.
    pub fn prev(&mut self) {
        if !self.valid {
            return;
        }

        let mut selected = self.selected;

        for _ in 0..self.count {
            // Si es 0, vuelve a la última posición para que no se rompa
            // el programa con un -1 (usize no puede ser negativo)
            if selected == 0 {
                selected = self.count.saturating_sub(1)
            } else {
                selected = selected.saturating_sub(1);
            }
            if !self.excluded.contains(&selected) {
                break;
            }
        }

        self.selected = selected
    }

    // ── Ajuste de rango ────────────────────────────────────────────────

    /// Adapta la selección cuando cambia la cantidad de elementos (`count`):
    /// si ahora hay 0 elementos, resetea el índice; si el elemento
    /// seleccionado quedó "afuera" del nuevo rango, lo mueve al último válido.
    pub fn clamp_seleted(&mut self, count: usize) {
        if count == 0 {
            self.reset();
        } else if self.valid {
            // → Selecciona el último elemento si el índice seleccionado es
            //   mayor a la cantidad de elementos actuales
            if self.selected > count.saturating_sub(1) {
                self.selected = count.saturating_sub(1);
            }
        }
    }
}

impl Default for SelectionIndex {
    /// Índice vacío y no válido (sin selección activa).
    fn default() -> Self {
        Self {
            selected: 0,
            count: 0,
            valid: false,
            excluded: Vec::default(),
        }
    }
}
