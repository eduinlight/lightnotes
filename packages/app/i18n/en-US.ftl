app-name = LightNotes

action-add = Add
action-cancel = Cancel
action-delete = Delete
action-close = Close
action-clear = Clear
action-manage = Manage
action-new-note = New note
action-log-in = Log in
action-sign-out = Sign out
action-previous = Previous
action-next = Next

sidebar-toggle = Toggle sidebar
sidebar-label = Sidebar
sidebar-description = Displays the mobile sidebar.

nav-primary = Primary
section-notes = Notes
section-diary = Diary
section-settings = Settings

filter-all-notes = All Notes
filter-starred = Starred
filter-pinned = Pinned

sync-saved = All changes saved
sync-offline = Offline — saved locally
sync-toggle-hint = Click to toggle offline

notes-count =
    { $count ->
        [one] { $count } note
       *[other] { $count } notes
    }
notes-search-title = Search
notes-search-results =
    { $count ->
        [one] { $count } result for “{ $query }”
       *[other] { $count } results for “{ $query }”
    }
notes-empty-no-matches = No matches
notes-empty-no-notes = No notes yet
notes-empty-search-hint = Try a different search term.
notes-empty-hint = Create your first note in this view.
notes-untitled = Untitled
notes-untitled-note = Untitled note
notes-no-additional-text = No additional text
notes-search-placeholder = Search notes…
notes-clear-search = Clear search

note-add-to-starred = Add to Starred
note-remove-from-starred = Remove from Starred
note-pin-to-top = Pin to top of list
note-unpin-from-top = Unpin from top
note-delete = Delete note
note-back-to-notes = Back to notes
note-edited = Edited { $time }
note-word-count =
    { $count ->
        [one] { $count } word
       *[other] { $count } words
    }
note-remove-tag = Remove tag
note-add-tag = + tag
note-add-tag-title = Add tag
note-new-tag-placeholder = New tag, then Enter
note-delete-title = Delete note?
note-delete-description = This note will be removed from every folder and tag. This can't be undone.

editor-bold = Bold
editor-italic = Italic
editor-code = Code
editor-inline-code = Inline code
editor-link = Link
editor-add-link-prompt = Add link…
editor-remove-link = Remove link
editor-paragraph = Paragraph
editor-paragraph-tooltip = Convert to plain paragraph
editor-heading-1 = Heading 1
editor-heading-2 = Heading 2
editor-heading-3 = Heading 3
editor-quote = Quote
editor-quote-tooltip = Toggle blockquote
editor-code-block = Code block
editor-bulleted-list = Bulleted list
editor-numbered-list = Numbered list
editor-lift-list = Lift out of list
editor-align-left = Align left
editor-align-center = Align center
editor-align-right = Align right
editor-justify = Justify
editor-justify-tooltip = Justify text
editor-uppercase = Uppercase
editor-uppercase-tooltip = Convert selection to UPPERCASE
editor-lowercase = Lowercase
editor-lowercase-tooltip = Convert selection to lowercase
editor-undo = Undo
editor-redo = Redo
editor-select-all = Select all
editor-table = Table
editor-insert-table = Insert 3x3 table
editor-insert-table-tooltip = Insert a 3x3 table at the cursor
editor-add-row = + Row
editor-add-row-tooltip = Insert a row after the current one
editor-add-column = + Col
editor-add-column-tooltip = Insert a column after the current one
editor-delete-row = − Row
editor-delete-row-tooltip = Delete the current row
editor-delete-column = − Col
editor-delete-column-tooltip = Delete the current column
editor-header-row = Header row
editor-header-row-tooltip = Toggle the first row as a header
editor-merge-row = Merge row
editor-merge-row-tooltip = Merge the selected cells across the current row
editor-merge-column = Merge column
editor-merge-column-tooltip = Merge the selected cells down the current column
editor-split-cell = Split cell
editor-split-cell-tooltip = Split a previously merged cell
editor-delete-table = Delete table
editor-delete-table-tooltip = Delete the whole table

link-dialog-title = Add link
link-dialog-description = Enter link text and a URL
link-dialog-text = Text
link-dialog-text-placeholder = Link text
link-dialog-url = URL
link-dialog-submit = Add link

folder-none = No folder
folder-move-to = Move to folder

folders-title = Folders
folders-manage-title = Manage folders
folders-manage-description = Create, rename, or delete folders
folders-hint =
    { $count ->
        [one] Rename in place, or delete a folder to move its notes to no folder. { $count } folder.
       *[other] Rename in place, or delete a folder to move its notes to no folder. { $count } folders.
    }
folders-new-placeholder = New folder name…
folders-empty = No folders yet.
folders-delete = Delete folder
folders-delete-title = Delete folder?
folders-delete-fallback-name = This folder
folders-delete-description = Its notes will move to no folder. This can't be undone.
folders-change-icon = Change icon

tags-title = Tags
tags-manage-title = Manage tags
tags-manage-description = Create, filter by, or delete tags
tags-hint =
    { $count ->
        [one] Create a tag, tap one to filter, or delete it everywhere. { $count } tag in use.
       *[other] Create a tag, tap one to filter, or delete it everywhere. { $count } tags in use.
    }
tags-new-placeholder = New tag name…
tags-empty = No tags yet.
tags-delete = Delete tag
tags-delete-title = Delete tag?
tags-delete-fallback-name = This tag
tags-delete-description = It will be removed from every note. This can't be undone.

diary-title = Diary
diary-empty-title = No note selected
diary-empty-description = Pick a day on the calendar, or write something new.
diary-nothing-written = Nothing written here yet.
diary-empty-note = Empty note
diary-filter-title = Filter
diary-filter-trigger = Filter by folder or tag
diary-all-folders = All folders
diary-all-tags = All tags
diary-today = Today

calendar-title = Calendar
calendar-day = Day
calendar-week = Week
calendar-month = Month

empty-state-title = No note selected
empty-state-description = Choose a note from the list, or create a new one to get started.

reminder-title = Reminder
reminder-remind-me = Remind me
reminder-at-the-time = At the time
reminder-none = No reminder
reminder-hours-before =
    { $count ->
        [one] { $count } hour before
       *[other] { $count } hours before
    }
reminder-days-before =
    { $count ->
        [one] { $count } day before
       *[other] { $count } days before
    }
reminder-weeks-before =
    { $count ->
        [one] { $count } week before
       *[other] { $count } weeks before
    }
reminder-short-hours = { $count }h before
reminder-short-days = { $count }d before
reminder-short-weeks = { $count }w before
reminder-fires = Fires { $when }
reminder-notification-untitled = Untitled note
reminder-notification-body = Due { $when }
reminder-notification-generic = You have a reminder
settings-reminders = Reminders
settings-reminders-enabled = Note reminders
settings-reminders-background-active = Fires even when LightNotes is closed
settings-reminders-background-unavailable = Only fires while LightNotes is open
settings-reminders-titles = Show note titles
settings-reminders-titles-description = Off keeps titles inside the encrypted store, and reminders just say a note is due
settings-reminders-permission = System notifications
settings-reminders-permission-granted = Allowed
settings-reminders-permission-denied = Blocked in your system settings
settings-reminders-permission-unknown = Not requested yet
settings-reminders-permission-unsupported = Not available on this platform
settings-reminders-permission-request = Allow

time-am = am
time-pm = pm
time-just-now = Just now
time-minutes-ago = { $count }m ago
time-hours-ago = { $count }h ago
time-yesterday = Yesterday
time-days-ago = { $count } days ago
time-weeks-ago =
    { $count ->
        [one] { $count } week ago
       *[other] { $count } weeks ago
    }

settings-title = Settings
settings-appearance = Appearance
settings-theme-dark = Dark
settings-theme-light = Light
settings-accent = Accent color
settings-accent-description = Used for highlights, links and controls
settings-language = Language
settings-language-description = Language used across the whole app
settings-sync = Sync & storage
settings-notes-stored =
    { $count ->
        [one] { $count } note stored locally · offline-ready
       *[other] { $count } notes stored locally · offline-ready
    }
settings-go-online = Go online
settings-go-offline = Go offline
settings-offline-storage = Offline storage
settings-offline-storage-description = Stored on this device
settings-editor = Editor
settings-editor-markdown = Live Markdown preview
settings-editor-folders-tags = Folders & tags
settings-editor-search = Full-text local search
settings-account = Account
settings-account-description = Sign in with Google to sync your notes across devices.
auth-signed-in-as = Signed in as
auth-not-signed-in = Not signed in
auth-sign-in-failed = Sign-in failed
auth-signing-in = Waiting for browser…
auth-sync-requires-sign-in = Sync is paused until you sign in.
auth-login-subtitle = Sign in with Google to reach your notes on every device.

settings-about = About
settings-version = Version 0.1.0 · local-first · multi-platform

language-en = English
language-es = Español

month-1 = January
month-2 = February
month-3 = March
month-4 = April
month-5 = May
month-6 = June
month-7 = July
month-8 = August
month-9 = September
month-10 = October
month-11 = November
month-12 = December

month-short-1 = Jan
month-short-2 = Feb
month-short-3 = Mar
month-short-4 = Apr
month-short-5 = May
month-short-6 = Jun
month-short-7 = Jul
month-short-8 = Aug
month-short-9 = Sep
month-short-10 = Oct
month-short-11 = Nov
month-short-12 = Dec

weekday-short-0 = Mon
weekday-short-1 = Tue
weekday-short-2 = Wed
weekday-short-3 = Thu
weekday-short-4 = Fri
weekday-short-5 = Sat
weekday-short-6 = Sun

weekday-narrow-0 = M
weekday-narrow-1 = T
weekday-narrow-2 = W
weekday-narrow-3 = T
weekday-narrow-4 = F
weekday-narrow-5 = S
weekday-narrow-6 = S
