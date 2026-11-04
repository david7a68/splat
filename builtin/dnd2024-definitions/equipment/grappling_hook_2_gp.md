---
splat_schema: 2
splat_id: a32df50c-196a-5842-8053-38461e70458a
splat_kind: item
splat_key: dnd2024:equipment/grappling_hook_2_gp
splat_name: Grappling Hook (2 GP)
splat_origin: dnd2024:reference/pages/097
splat_sources:
- SRD 5.2.1, pp. 97, https://www.dndbeyond.com/srd
---

# Grappling Hook (2 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 97 -->

As a Utilize action, you can throw the Grappling Hook at a railing, a ledge, or another catch within 50 feet of yourself, and the hook catches on if you succeed on a DC 13 Dexterity (Acrobatics) check. If you tied a Rope to the hook, you can then climb it.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/grappling_hook_2_gp ` |
| ` implementation ` | ` dnd2024:equipment_grappling_hook_2_gp ` |

### Parameters

#### Throw

<!-- splat:yaml path=splat_rule/parameters/throw -->

```yaml
action: utilize
range_ft: 50
target_kinds:
- railing
- ledge
- other_catch
check:
  ability: dexterity
  skill: acrobatics
  dc: 13
success: hook_catches
```

#### Climb

<!-- splat:values path=splat_rule/parameters/climb -->

| Field | Value |
| --- | --- |
| ` requires ` | ` rope_tied_to_caught_hook ` |
| ` operation ` | ` climb_rope ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 4.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 2
> mass:
>   unit: lb
>   value: 4.0
> ```

