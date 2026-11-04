---
splat_schema: 2
splat_id: 0375c0dd-6a44-4459-88ad-f23d9f156e07
splat_kind: item
splat_key: "my_homebrew:items/backpack"
splat_name: Training Backpack
---

The fixture constrains contents by mass only. No volume limit is inferred.


## Item

| Field | Value |
| --- | --- |
| Type | Container |
| Rarity |  |
| Attunement | No |


> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> containment:
>   capacity:
>     mass:
>       value: 30
>       unit: lb
>   carried_mass: self_and_contents
>   access:
>     mechanic: dnd2024:container_access
>     state: closed
> mass:
>   value: 5
>   unit: lb
> stackable: false
> equipment:
>   allowed:
>   - carried
>   - stored
>   - worn
> ```

