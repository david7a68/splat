---
splat_schema: 2
splat_id: 5cf2e046-608a-5241-aebe-1cc8b112d6e6
splat_kind: spell
splat_key: dnd2024:spells/clone
splat_name: Clone
splat_origin: dnd2024:reference/spell/clone
splat_sources:
- SRD 5.2.1, pp. 115, 116, https://www.dndbeyond.com/srd
---

# Clone


## Spell

| Field | Value |
| --- | --- |
| Level | 8 |
| School | Necromancy |
| Classes | Wizard |
| Casting Time | 1 hour |
| Range |  |
| Components | V, S, M (a diamond worth 1,000+ GP, which the spell consumes, and a sealable vessel worth 2,000+ GP that is large enough to hold the creature being cloned) |
| Duration | Instantaneous |


## Source text

<!-- SRD page 115 -->

Level 8 Necromancy (Wizard)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (a diamond worth 1,000+ GP, which the spell consumes, and a sealable vessel worth 2,000+ GP that is large enough to hold the creature being cloned)\
Duration: Instantaneous

<!-- SRD page 116 -->

You touch a creature or at least 1 cubic inch of its flesh. An inert duplicate of that creature forms inside the vessel used in the spell’s casting and finishes growing after 120 days; you choose whether the finished clone is the same age as the creature or younger. The clone remains inert and endures indefinitely while its vessel remains undisturbed.

If the original creature dies after the clone finishes forming, the creature’s soul transfers to the clone if the soul is free and willing to return. The clone is physically identical to the original and has the same personality, memories, and abilities, but none of the original’s equipment. The creature’s original remains, if any, become inert and can’t be revived, since the creature’s soul is elsewhere.

Original layout: [[../../srd-5.2.1/pages/115|page 115]], [[../../srd-5.2.1/pages/116|page 116]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/clone ` |
| ` implementation ` | ` dnd2024:spell_clone ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` touch ` | ` true ` |

##### Options

<!-- splat:table
path: splat_rule/parameters/targeting/options
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
  Volume min cubic inches:
    path: volume_min_cubic_inches
    type: integer
    optional: true
-->

| Key | Kind | Volume min cubic inches |
| --- | --- | --- |
| ` row_1 ` | ` Creature ` |  |
| ` row_2 ` | ` Flesh sample ` | ` 1 ` |

#### Duplicate

<!-- splat:yaml path=splat_rule/parameters/duplicate -->

```yaml
location: component_vessel
maturity_days: 120
age_choice:
- same_as_original
- younger
before_transfer: inert
preservation:
  duration: indefinite
  requires: vessel_undisturbed
```

#### Soul transfer

<!-- splat:yaml path=splat_rule/parameters/soul_transfer -->

```yaml
event: original.died
requires:
- clone_mature
- soul_free
- soul_willing
destination: clone
preserved:
- physical_identity
- personality
- memories
- abilities
equipment: none
original_remains:
  inert: true
  revival_possible: false
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: true
> range:
>   mode: touch
> ```

