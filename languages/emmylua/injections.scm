;; Injections for Lua code

;; Comment
((comment_content) @injection.content
  (#set! injection.language "comment")
)

;; EmmyLua Doc comments injection
;; Inject --- comments as emmyluadoc language
;; The emmyluadoc grammar now supports --- prefix, so we can directly inject
(((comment) @_emmyluadoc_comment
  (#match? @_emmyluadoc_comment "^---")) @injection.content
  (#set! injection.language "emmyluadoc"))

;; LuaJIT FFI C code injection
((function_call
  name: [
    (identifier) @_cdef_identifier
    (_ _ (identifier) @_cdef_identifier)
  ]
  arguments: (arguments (string content: _ @injection.content
    (#set! injection.language "c"))))
  (#eq? @_cdef_identifier "cdef"))
