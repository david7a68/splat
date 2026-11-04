---
splat_schema: 2
splat_id: 4d28de7e-cb68-587c-8897-1ccc428b74a1
splat_kind: spell
splat_key: dnd2024:spells/remove_curse
splat_name: Remove Curse
splat_origin: dnd2024:reference/spell/remove_curse
splat_sources:
- SRD 5.2.1, pp. 158, https://www.dndbeyond.com/srd
---

# Remove Curse


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Abjuration |
| Classes | Cleric, Paladin, Warlock, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Instantaneous |


## Source text

<!-- SRD page 158 -->

Level 3 Abjuration (Cleric, Paladin, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous

At your touch, all curses affecting one creature or object end. If the object is a cursed magic item, its curse remains, but the spell breaks its owner’s Attunement to the object so it can be removed or discarded.

Original layout: [[../../srd-5.2.1/pages/158|page 158]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/remove_curse ` |
| ` implementation ` | ` dnd2024:spell_remove_curse ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kinds ` | ` ["creature", "object"] ` |
| ` touch ` | ` true ` |
| ` count ` | ` 1 ` |

#### Normal

<!-- splat:values path=splat_rule/parameters/normal -->

| Field | Value |
| --- | --- |
| ` end ` | ` all_curses_affecting_target ` |

#### Cursed magic item exception

<!-- splat:values path=splat_rule/parameters/cursed_magic_item_exception -->

| Field | Value |
| --- | --- |
| ` item_curse ` | ` remains ` |
| ` owner_attunement ` | ` broken ` |
| ` item_removal_and_discard ` | ` permitted ` |

#### Target binding

<!-- splat:values path=splat_rule/parameters/target_binding -->

| Field | Value |
| --- | --- |
| ` exactly_one ` | ` ["target_creature", "target_object"] ` |
| ` must_match_selected_mode ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: touch
> ```

