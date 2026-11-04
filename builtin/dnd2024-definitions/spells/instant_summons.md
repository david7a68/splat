---
splat_schema: 2
splat_id: 04c83d44-8888-569d-93c9-6d1dd55650c0
splat_kind: spell
splat_key: dnd2024:spells/instant_summons
splat_name: Instant Summons
splat_origin: dnd2024:reference/spell/instant_summons
splat_sources:
- SRD 5.2.1, pp. 143, https://www.dndbeyond.com/srd
---

# Instant Summons


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Conjuration |
| Classes | Wizard |
| Casting Time | 1 minute or Ritual |
| Range |  |
| Components | V, S, M (a sapphire worth 1,000+ GP) |
| Duration | Until dispelled |


## Source text

<!-- SRD page 143 -->

Level 6 Conjuration (Wizard)\
Casting Time: 1 minute or Ritual\
Range: Touch\
Components: V, S, M (a sapphire worth 1,000+ GP)\
Duration: Until dispelled

You touch the sapphire used in the casting and an object weighing 10 pounds or less whose longest dimension is 6 feet or less. The spell leaves an Invisible mark on that object and invisibly inscribes the object’s name on the sapphire. Each time you cast this spell, you must use a different sapphire.

Thereafter, you can take a Magic action to speak the object’s name and crush the sapphire. The object instantly appears in your hand regardless of physical or planar distances, and the spell ends.

If another creature is holding or carrying the object, crushing the sapphire doesn’t transport it, but instead you learn who that creature is and where that creature is currently located.

Original layout: [[../../srd-5.2.1/pages/143|page 143]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/instant_summons ` |
| ` implementation ` | ` dnd2024:spell_instant_summons ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` object ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` touch ` | ` true ` |
| ` maximum_weight_pounds ` | ` 10 ` |
| ` maximum_dimension_feet ` | ` 6 ` |

#### Sapphire

<!-- splat:values path=splat_rule/parameters/sapphire -->

| Field | Value |
| --- | --- |
| ` touch ` | ` true ` |
| ` different_each_cast ` | ` true ` |
| ` invisible_inscription ` | ` object_name ` |

#### Mark

<!-- splat:values path=splat_rule/parameters/mark -->

| Field | Value |
| --- | --- |
| ` invisible ` | ` true ` |
| ` "on" ` | ` object ` |

#### Summon

<!-- splat:yaml path=splat_rule/parameters/summon -->

```yaml
action: magic
requirements:
- speak_object_name
- crush_sapphire
range: any_physical_or_planar_distance
success:
  move_object_to: caster_hand
  end_spell: true
held_or_carried_by_other_creature:
  transport: false
  reveal:
  - holder_identity
  - holder_current_location
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

