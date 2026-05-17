# Capítulo 8: Modelo de Gobernanza

## 8.1 La lógica del poder de voto

En la era post-monetaria, el poder de voto en la gobernanza social no puede basarse en la riqueza (la moneda ha dejado de ser efectiva), ni debería basarse en la autoridad (lo cual viola los principios de descentralización).

La respuesta de GMC: **El poder de voto se deriva de la participación de contribuciones de cada uno dentro de una comunidad.**

Esto significa:
- Cuanto más contribuyes y mayor es tu reputación, mayor es tu influencia
- El poder de voto es dinámico, fluctuando con el decaimiento y crecimiento de MeriToken
- Sin contribuciones sostenidas, la influencia se desvanece naturalmente — no existen privilegios permanentes

## 8.2 Mecanismo de votación ponderada

```
Votos efectivos individuales = Votos base × (MeriToken individual / MeriToken total de la comunidad)
```

Todos tienen derecho a votar (votos base = 1), pero el peso es proporcional a la participación de MeriToken de cada uno.

### Ejemplo

Una comunidad tiene 3 miembros:

| Miembro | MeriToken | Participación | Votos efectivos |
|---------|-----------|---------------|-----------------|
| A | 100 | 50% | 0.5 |
| B | 60 | 30% | 0.3 |
| C | 40 | 20% | 0.2 |

A + C votan a favor, B vota en contra: A favor 0.7 > En contra 0.3 → Aprobado.

## 8.3 Escenarios de gobernanza

| Escenario | Votantes | Condición de aprobación | Notas |
|-----------|----------|------------------------|-------|
| Reconocimiento de contribución | Partes interesadas (excluyendo alta intimidad) | Mayoría de 2/3 | Operación rutinaria |
| Decisión de sanción | Partes interesadas afectadas | Mayoría de 3/4 | Comportamiento severo requiere un umbral más alto |
| Cambio de reglas | Todos los miembros de la comunidad | Mayoría absoluta de 2/3 | Afecta a todos |

## 8.4 Comunidades

Las comunidades son las unidades de gobernanza en GMC:

- Una persona puede pertenecer a múltiples comunidades
- Las comunidades pueden anidarse (subcomunidades)
- El poder de voto se calcula independientemente en cada comunidad
- La misma persona puede tener niveles de influencia completamente diferentes en diferentes comunidades

## 8.5 Anti-monopolio

La participación de MeriToken determina el poder de voto, pero debe prevenirse la concentración extrema:

- **El mecanismo de decaimiento en sí es anti-monopolio**: sin contribuciones sostenidas, se pierde el poder de voto
- **Estratificación comunitaria**: en comunidades grandes, las participaciones individuales se diluyen naturalmente
- **Participación en lugar de valor absoluto**: los aumentos en la oferta total no afectan la equidad de la gobernanza

## 8.6 Gobernanza colaborativa humano-IA

- El voto de un iFay representa la voluntad de su arquetipo humano
- El voto de un coFay representa la voluntad de su organización afiliada
- Todo comportamiento de votación es transparente y auditable en cadena
- Humanos y Fays operan dentro del mismo marco de gobernanza

## 8.7 Notas de discusión

> Decisiones de diseño para el modelo de gobernanza:
> - "Ponderado por participación" en lugar de "una persona, un voto": el principio central es "las contribuciones determinan el poder de voto"
> - "Participación" en lugar de "valor absoluto": previene que los participantes tempranos monopolicen permanentemente la influencia
> - El decaimiento es una salvaguarda natural para la equidad de la gobernanza
> - Puede ser necesario un mecanismo de "tope de poder de voto" en el futuro para prevenir el control absoluto por una sola entidad en comunidades pequeñas
