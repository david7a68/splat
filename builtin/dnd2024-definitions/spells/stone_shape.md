---
splat_schema: 2
splat_id: 2de9b05c-d0ae-535b-a710-0af8929e509f
splat_kind: spell
splat_key: dnd2024:spells/stone_shape
splat_name: Stone Shape
splat_origin: dnd2024:reference/spell/stone_shape
splat_sources:
- SRD 5.2.1, pp. 165, https://www.dndbeyond.com/srd
---

# Stone Shape


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Transmutation |
| Classes | Cleric, Druid, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (soft clay) |
| Duration | Instantaneous |


## Source text

<!-- SRD page 165 -->

Level 4 Transmutation (Cleric, Druid, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (soft clay)\
Duration: Instantaneous

You touch a stone object of Medium size or smaller or a section of stone no more than 5 feet in any dimension and form it into any shape you like. For example, you could shape a large rock into a weapon, statue, or coffer, or you could make a small passage through a wall that is 5 feet thick. You could also shape a stone door or its frame to seal the door shut. The object you create can have up to two hinges and a latch, but finer mechanical detail isn’t possible.

Original layout: [[../../srd-5.2.1/pages/165|page 165]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/stone_shape ` |
| ` implementation ` | ` dnd2024:spell_stone_shape ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` touch ` | ` true ` |

##### Choice

<!-- splat:table
path: splat_rule/parameters/target/choice
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Kind:
    path: kind
    type: symbol
  Maximum size:
    path: maximum_size
    type: symbol
    optional: true
  Maximum each dimension feet:
    path: maximum_each_dimension_feet
    type: integer
    optional: true
-->

| Key | Kind | Maximum size | Maximum each dimension feet |
| --- | --- | --- | --- |
| ` row_1 ` | ` Stone object ` | ` Medium ` |  |
| ` row_2 ` | ` Stone section ` |  | ` 5 ` |

#### Shape

<!-- splat:values path=splat_rule/parameters/shape -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` arbitrary_shape ` | ` true ` |
| ` maximum_hinges ` | ` 2 ` |
| ` latch_allowed ` | ` true ` |
| ` finer_mechanical_detail ` | ` false ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

