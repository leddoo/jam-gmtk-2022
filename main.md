todo:
- polish:
    - goal highlight.
    - goal hit effect.
    - win effect.
    - level transitions.
    - tutorial overlay for first level: key guide for moves, flag on target.
    - move buffering.
    - moves:
        - invalid move feedback.
        - pulsing blue outline.
- level design.
    - multiple in a row.
- parser: begin/end markers, pad right with space.


polish:
- draw valid moves only.
- invalid move feedback.
- draw "thick line around tail".
- use texture for dice eyes.
- sound effects.
- rotate eyes?
- menu?
- don't forget to remove level skip keys.


- polish & accessibility.
    - key repeat / move buffering.
        - if key went down, add to buffer if not full.
        - if key is down and buffer is empty, add to buffer.
        - on invalid move, flush buffer.

ideas:
- puzzle game where dice leaves face prints
    - can use dice theme for ui.
    - level ideas:
        - there and back.
        - line up, then multiple in a row.
        - multiple in a row, but you have to not do it linearly.
        - explore: the sides stay fixed while moving in one line.
            - how can you move in a line while swapping the side faces?
            - is a strategy to align, then move long distance.

