---
splat_schema: 2
splat_id: 64fc0893-46bb-5a96-a967-3e00f78b49c4
splat_kind: spell
splat_key: dnd2024:spells/maze
splat_name: Maze
splat_origin: dnd2024:reference/spell/maze
splat_sources:
- SRD 5.2.1, pp. 148, https://www.dndbeyond.com/srd
---

# Maze


## Spell

| Field | Value |
| --- | --- |
| Level | 8 |
| School | Conjuration |
| Classes | Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S |
| Duration | Concentration, up to 10 minutes |


## Source text

<!-- SRD page 148 -->

Level 8 Conjuration (Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes

You banish a creature that you can see within range into a labyrinthine demiplane. The target remains there for the duration or until it escapes the maze.

The target can take a Study action to try to escape. When it does so, it makes a DC 20 Intelligence (Investigation) check. If it succeeds, it escapes, and the spell ends.

When the spell ends, the target reappears in the space it left or, if that space is occupied, in the nearest unoccupied space.

Original layout: [[../../srd-5.2.1/pages/148|page 148]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/maze ` |
| ` implementation ` | ` dnd2024:spell_maze ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 600 ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` visible ` | ` true ` |
| ` range_feet ` | ` 60 ` |

#### Banish

<!-- splat:values path=splat_rule/parameters/banish -->

| Field | Value |
| --- | --- |
| ` destination ` | ` labyrinthine_demiplane ` |
| ` initial_save ` | ` false ` |

#### Escape

<!-- splat:yaml path=splat_rule/parameters/escape -->

```yaml
economy: study_action
check:
  ability: intelligence
  skill: investigation
  dc: 20
success:
  escape: true
  end_spell: true
```

#### On spell end

<!-- splat:values path=splat_rule/parameters/on_spell_end -->

| Field | Value |
| --- | --- |
| ` return ` | ` original_space ` |
| ` occupied_fallback ` | ` nearest_unoccupied_space ` |
