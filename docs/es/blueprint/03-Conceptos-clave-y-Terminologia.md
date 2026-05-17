# Capítulo 3: Conceptos clave y Terminología

## 3.1 Visión general de relaciones entre entidades

```
Persona natural (arquetipo humano)
  └── HumanID (globalmente único)
        ├── iFay-1 (vinculado permanentemente)
        │     ├── FayID
        │     └── MeritPocket → MeriToken (múltiples lotes)
        ├── iFay-2 (vinculado permanentemente)
        │     ├── FayID
        │     └── MeritPocket → MeriToken (múltiples lotes)
        └── ...

Organización / Individuo
  └── coFay (relación de propiedad, transferible)
        ├── FayID
        └── MeritPocket → MeriToken (múltiples lotes)
```

## 3.2 Terminología central

### MeriToken

La unidad cuantitativa de contribución. Representa la credibilidad social y el poder de voto de una entidad.

- No negociable, no transferible
- Decae con el tiempo siguiendo una curva exponencial
- Tiene un valor mínimo que no puede llegar a cero
- Puede heredarse bajo reglas estrictas (con atenuación)

### MeritPocket

El contenedor de MeriToken, análogo a una billetera. Cada Fay está vinculado a un MeritPocket.

### iFay (Fay personal)

Un agente personal de IA — la "armadura digital". Vinculado permanentemente a una persona natural; no puede desvincularse.

- Esencia: Una extensión de la personalidad, no un activo
- El MeriToken generado por un iFay pertenece a su arquetipo humano
- Una persona puede tener múltiples iFay

### coFay (Fay organizacional)

Un agente de IA organizacional o comercial. Pertenece a un individuo u organización.

- Esencia: Un activo, transferible
- El MeriToken generado por un coFay pertenece a su propietario actual
- Tras la transferencia, el MeritPocket se transfiere con él; el MeriToken no se atenúa

### Arquetipo humano

La persona natural a la que un iFay está vinculado permanentemente. Cada arquetipo humano posee un HumanID único.

### HumanID / FayID

- HumanID: Un identificador de identidad humana globalmente único
- FayID: Un identificador de identidad Fay globalmente único
- Un HumanID puede corresponder a múltiples FayIDs
- HumanID y FayID aparecen en pares

### Lote de Merit

La unidad de registro para cada adquisición de contribución, que contiene: cantidad adquirida, duración de influencia, parámetros de decaimiento y tiempo de adquisición.

### Partes interesadas

Partes con un interés en una contribución determinada. Responsables del voto por consenso sobre contribuciones; seleccionadas excluyendo a individuos con intimidad excesivamente alta con el contribuyente.

### Cementerio digital

El estado en el que un iFay puede ser colocado después de que su arquetipo humano fallezca. Un iFay en el cementerio digital puede seguir teniendo interacciones pasivas, pero todas las acciones se etiquetan como "desde el cementerio digital".

## 3.3 Diferencias esenciales entre iFay y coFay

| Dimensión | iFay | coFay |
|-----------|------|-------|
| Esencia | Extensión de la personalidad | Activo |
| Relación de vinculación | Vinculado permanentemente, no puede desvincularse | Relación de propiedad, transferible |
| Tras el fallecimiento del arquetipo humano | Entra en tutela o cementerio digital | Se hereda/transfiere como un activo |
| MeriToken tras transferencia | No puede transferirse | Se transfiere con el coFay, sin atenuación |
| Número de propietarios | Pertenece a una sola persona natural | Pertenece a un individuo u organización |

## 3.4 MeriToken y Soulbound Token (SBT)

SBT es un concepto propuesto por Vitalik Buterin en 2022 — un Token no transferible vinculado a una identidad específica, utilizado para representar atributos que no deberían ser negociados (credenciales, reputación, logros).

MeriToken es una versión mejorada de SBT:

| Característica | SBT estándar | MeriToken |
|----------------|--------------|-----------|
| No transferible | ✓ | ✓ |
| Método de vinculación | Vinculado a dirección de billetera | Vinculado a iFay → MeritPocket → arquetipo humano |
| Dimensión temporal | Ninguna (permanentemente válido) | Sí (decaimiento exponencial) |
| Heredable | No | Sí (con atenuación) |
| Método de cuantificación | Típicamente booleano (tiene/no tiene) | Valor numérico continuo |
| Garantía de mínimo | Ninguna | Sí (minMerit) |

## 3.5 Notas de discusión

> Lógica de diseño del sistema terminológico:
> - La vinculación de tres capas (arquetipo humano → iFay → MeritPocket) aísla las capas de identidad, agente y activos
> - iFay como "extensión de la personalidad" es no transferible, asegurando que la reputación sea inseparable de la persona
> - coFay como "activo" es transferible, asegurando la flexibilidad operativa organizacional
> - MeriToken hace referencia a SBT pero añade decaimiento temporal y heredabilidad, haciéndolo más adecuado para escenarios de medición dinámica de contribuciones
