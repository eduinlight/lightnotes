app-name = LightNotes

action-add = Añadir
action-cancel = Cancelar
action-delete = Eliminar
action-close = Cerrar
action-clear = Limpiar
action-manage = Gestionar
action-new-note = Nueva nota
action-log-in = Iniciar sesión
action-sign-out = Cerrar sesión
action-previous = Anterior
action-next = Siguiente

sidebar-toggle = Mostrar u ocultar la barra lateral
sidebar-label = Barra lateral
sidebar-description = Muestra la barra lateral en móvil.

nav-primary = Principal
section-notes = Notas
section-diary = Diario
section-settings = Ajustes

filter-all-notes = Todas las notas
filter-starred = Destacadas
filter-pinned = Fijadas

sync-saved = Todos los cambios guardados
sync-offline = Sin conexión — guardado localmente
sync-toggle-hint = Haz clic para cambiar el modo sin conexión

notes-count =
    { $count ->
        [one] { $count } nota
       *[other] { $count } notas
    }
notes-search-title = Búsqueda
notes-search-results =
    { $count ->
        [one] { $count } resultado para “{ $query }”
       *[other] { $count } resultados para “{ $query }”
    }
notes-empty-no-matches = Sin coincidencias
notes-empty-no-notes = Aún no hay notas
notes-empty-search-hint = Prueba con otro término de búsqueda.
notes-empty-hint = Crea tu primera nota en esta vista.
notes-untitled = Sin título
notes-untitled-note = Nota sin título
notes-no-additional-text = Sin texto adicional
notes-search-placeholder = Buscar notas…
notes-clear-search = Limpiar búsqueda

note-add-to-starred = Añadir a Destacadas
note-remove-from-starred = Quitar de Destacadas
note-pin-to-top = Fijar al principio de la lista
note-unpin-from-top = Dejar de fijar
note-delete = Eliminar nota
note-back-to-notes = Volver a las notas
note-edited = Editada { $time }
note-word-count =
    { $count ->
        [one] { $count } palabra
       *[other] { $count } palabras
    }
note-remove-tag = Quitar etiqueta
note-add-tag = + etiqueta
note-add-tag-title = Añadir etiqueta
note-new-tag-placeholder = Nueva etiqueta y pulsa Enter
note-delete-title = ¿Eliminar la nota?
note-delete-description = Esta nota se quitará de todas las carpetas y etiquetas. No se puede deshacer.

editor-bold = Negrita
editor-italic = Cursiva
editor-code = Código
editor-inline-code = Código en línea
editor-link = Enlace
editor-add-link-prompt = Añadir enlace…
editor-remove-link = Quitar enlace
editor-paragraph = Párrafo
editor-paragraph-tooltip = Convertir en párrafo simple
editor-heading-1 = Encabezado 1
editor-heading-2 = Encabezado 2
editor-heading-3 = Encabezado 3
editor-quote = Cita
editor-quote-tooltip = Alternar cita en bloque
editor-code-block = Bloque de código
editor-bulleted-list = Lista con viñetas
editor-numbered-list = Lista numerada
editor-lift-list = Sacar de la lista
editor-align-left = Alinear a la izquierda
editor-align-center = Centrar
editor-align-right = Alinear a la derecha
editor-justify = Justificar
editor-justify-tooltip = Justificar el texto
editor-uppercase = Mayúsculas
editor-uppercase-tooltip = Convertir la selección a MAYÚSCULAS
editor-lowercase = Minúsculas
editor-lowercase-tooltip = Convertir la selección a minúsculas
editor-undo = Deshacer
editor-redo = Rehacer
editor-select-all = Seleccionar todo
editor-table = Tabla
editor-insert-table = Insertar tabla 3x3
editor-insert-table-tooltip = Insertar una tabla 3x3 en el cursor
editor-add-row = + Fila
editor-add-row-tooltip = Insertar una fila después de la actual
editor-add-column = + Col
editor-add-column-tooltip = Insertar una columna después de la actual
editor-delete-row = − Fila
editor-delete-row-tooltip = Eliminar la fila actual
editor-delete-column = − Col
editor-delete-column-tooltip = Eliminar la columna actual
editor-header-row = Fila de encabezado
editor-header-row-tooltip = Usar la primera fila como encabezado
editor-merge-row = Combinar fila
editor-merge-row-tooltip = Combinar las celdas seleccionadas de la fila actual
editor-merge-column = Combinar columna
editor-merge-column-tooltip = Combinar las celdas seleccionadas de la columna actual
editor-split-cell = Dividir celda
editor-split-cell-tooltip = Dividir una celda combinada previamente
editor-delete-table = Eliminar tabla
editor-delete-table-tooltip = Eliminar toda la tabla

link-dialog-title = Añadir enlace
link-dialog-description = Escribe el texto del enlace y una URL
link-dialog-text = Texto
link-dialog-text-placeholder = Texto del enlace
link-dialog-url = URL
link-dialog-submit = Añadir enlace

folder-none = Sin carpeta
folder-move-to = Mover a la carpeta

folders-title = Carpetas
folders-manage-title = Gestionar carpetas
folders-manage-description = Crea, renombra o elimina carpetas
folders-hint =
    { $count ->
        [one] Renombra en el momento o elimina una carpeta para dejar sus notas sin carpeta. { $count } carpeta.
       *[other] Renombra en el momento o elimina una carpeta para dejar sus notas sin carpeta. { $count } carpetas.
    }
folders-new-placeholder = Nombre de la nueva carpeta…
folders-empty = Aún no hay carpetas.
folders-delete = Eliminar carpeta
folders-delete-title = ¿Eliminar la carpeta?
folders-delete-fallback-name = Esta carpeta
folders-delete-description = Sus notas quedarán sin carpeta. No se puede deshacer.
folders-change-icon = Cambiar icono

tags-title = Etiquetas
tags-manage-title = Gestionar etiquetas
tags-manage-description = Crea, filtra o elimina etiquetas
tags-hint =
    { $count ->
        [one] Crea una etiqueta, toca una para filtrar o elimínala de todas partes. { $count } etiqueta en uso.
       *[other] Crea una etiqueta, toca una para filtrar o elimínala de todas partes. { $count } etiquetas en uso.
    }
tags-new-placeholder = Nombre de la nueva etiqueta…
tags-empty = Aún no hay etiquetas.
tags-delete = Eliminar etiqueta
tags-delete-title = ¿Eliminar la etiqueta?
tags-delete-fallback-name = Esta etiqueta
tags-delete-description = Se quitará de todas las notas. No se puede deshacer.

diary-title = Diario
diary-empty-title = Ninguna nota seleccionada
diary-empty-description = Elige un día en el calendario o escribe algo nuevo.
diary-nothing-written = Aún no hay nada escrito aquí.
diary-empty-note = Nota vacía
diary-filter-title = Filtro
diary-filter-trigger = Filtrar por carpeta o etiqueta
diary-all-folders = Todas las carpetas
diary-all-tags = Todas las etiquetas
diary-today = Hoy

calendar-title = Calendario
calendar-day = Día
calendar-week = Semana
calendar-month = Mes

empty-state-title = Ninguna nota seleccionada
empty-state-description = Elige una nota de la lista o crea una nueva para empezar.

reminder-title = Recordatorio
reminder-remind-me = Recordármelo
reminder-at-the-time = A la hora
reminder-none = Sin recordatorio
reminder-hours-before =
    { $count ->
        [one] { $count } hora antes
       *[other] { $count } horas antes
    }
reminder-days-before =
    { $count ->
        [one] { $count } día antes
       *[other] { $count } días antes
    }
reminder-weeks-before =
    { $count ->
        [one] { $count } semana antes
       *[other] { $count } semanas antes
    }
reminder-short-hours = { $count } h antes
reminder-short-days = { $count } d antes
reminder-short-weeks = { $count } sem antes
reminder-fires = Suena { $when }
reminder-notification-untitled = Nota sin título
reminder-notification-body = Vence { $when }
reminder-notification-generic = Tienes un recordatorio
settings-reminders = Recordatorios
settings-reminders-enabled = Recordatorios de notas
settings-reminders-background-active = Suenan aunque LightNotes esté cerrado
settings-reminders-background-unavailable = Solo suenan mientras LightNotes está abierto
settings-reminders-titles = Mostrar títulos de notas
settings-reminders-titles-description = Al desactivarlo los títulos no salen del almacén cifrado y el recordatorio solo avisa de que hay una nota
settings-reminders-permission = Notificaciones del sistema
settings-reminders-permission-granted = Permitidas
settings-reminders-permission-denied = Bloqueadas en los ajustes del sistema
settings-reminders-permission-unknown = Aún sin solicitar
settings-reminders-permission-unsupported = No disponible en esta plataforma
settings-reminders-permission-request = Permitir

time-am = a. m.
time-pm = p. m.
time-just-now = Ahora mismo
time-minutes-ago = hace { $count } min
time-hours-ago = hace { $count } h
time-yesterday = Ayer
time-days-ago = hace { $count } días
time-weeks-ago =
    { $count ->
        [one] hace { $count } semana
       *[other] hace { $count } semanas
    }

settings-title = Ajustes
settings-appearance = Apariencia
settings-theme-dark = Oscuro
settings-theme-light = Claro
settings-accent = Color de acento
settings-accent-description = Se usa en resaltados, enlaces y controles
settings-language = Idioma
settings-language-description = Idioma usado en toda la aplicación
settings-sync = Sincronización y almacenamiento
settings-notes-stored =
    { $count ->
        [one] { $count } nota guardada localmente · lista sin conexión
       *[other] { $count } notas guardadas localmente · listas sin conexión
    }
settings-go-online = Conectarse
settings-go-offline = Desconectarse
settings-offline-storage = Almacenamiento sin conexión
settings-offline-storage-description = Guardado en este dispositivo
settings-editor = Editor
settings-editor-markdown = Vista previa de Markdown en vivo
settings-editor-folders-tags = Carpetas y etiquetas
settings-editor-search = Búsqueda local de texto completo
settings-account = Cuenta
settings-account-description = Inicia sesión con Google para sincronizar tus notas entre dispositivos.
auth-signed-in-as = Sesión iniciada como
auth-not-signed-in = Sin sesión iniciada
auth-sign-in-failed = Error al iniciar sesión
auth-signing-in = Esperando al navegador…
auth-sync-requires-sign-in = La sincronización está pausada hasta que inicies sesión.
auth-login-subtitle = Inicia sesión con Google para acceder a tus notas en todos tus dispositivos.

settings-about = Acerca de
settings-version = Versión 0.1.0 · local-first · multiplataforma

language-en = English
language-es = Español

month-1 = enero
month-2 = febrero
month-3 = marzo
month-4 = abril
month-5 = mayo
month-6 = junio
month-7 = julio
month-8 = agosto
month-9 = septiembre
month-10 = octubre
month-11 = noviembre
month-12 = diciembre

month-short-1 = ene
month-short-2 = feb
month-short-3 = mar
month-short-4 = abr
month-short-5 = may
month-short-6 = jun
month-short-7 = jul
month-short-8 = ago
month-short-9 = sep
month-short-10 = oct
month-short-11 = nov
month-short-12 = dic

weekday-short-0 = lun
weekday-short-1 = mar
weekday-short-2 = mié
weekday-short-3 = jue
weekday-short-4 = vie
weekday-short-5 = sáb
weekday-short-6 = dom

weekday-narrow-0 = L
weekday-narrow-1 = M
weekday-narrow-2 = X
weekday-narrow-3 = J
weekday-narrow-4 = V
weekday-narrow-5 = S
weekday-narrow-6 = D
