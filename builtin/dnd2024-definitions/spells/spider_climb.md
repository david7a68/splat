---
splat_schema: 2
splat_id: c0b0eb2c-04e7-5e92-932b-2ad461c3a302
splat_kind: spell
splat_key: dnd2024:spells/spider_climb
splat_name: Spider Climb
splat_origin: dnd2024:reference/spell/spider_climb
splat_sources:
- SRD 5.2.1, pp. 164, https://www.dndbeyond.com/srd
---

# Spider Climb


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Transmutation |
| Classes | Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a drop of bitumen and a spider) |
| Duration | Concentration, up to 1 hour |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 164 -->

Level 2 Transmutation (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a drop of bitumen and a spider)\
Duration: Concentration, up to 1 hour

Until the spell ends, one willing creature you touch gains the ability to move up, down, and across vertical surfaces and along ceilings, while leaving its hands free. The target also gains a Climb Speed equal to its Speed.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/164|page 164]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/spider_climb ` |
| ` implementation ` | ` dnd2024:spell_spider_climb ` |

### Parameters

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
kind: creature
maximum:
  op: add
  args:
  - 1
  - op: multiply
    args:
    - 1
    - op: subtract
      args:
      - binding: cast_level
      - 2
within_spell_range: true
willing: true
touched: true
```

#### Movement

<!-- splat:values path=splat_rule/parameters/movement -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |
| ` surfaces ` | ` ["vertical_up", "vertical_down", "vertical_across", "ceilings"] ` |
| ` hands_remain_free ` | ` true ` |
| ` climb_speed ` | ` target_speed ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

