# Capítulo 13: Modelo Económico y Lógica de Acuñación

## 13.1 MeriToken no es una moneda

Para reiterar el posicionamiento económico de MeriToken:

- No negociable, no transferible, no liquidable (sin mercado secundario)
- Sin valor especulativo
- No es un medio de intercambio
- Puramente una medida de contribución y un portador de poder de voto

Por lo tanto, las restricciones de la economía monetaria tradicional (control de inflación, política monetaria) no se aplican a MeriToken.

## 13.2 Selección del enfoque de acuñación

> Se evaluaron tres enfoques durante la discusión:

| Enfoque | Descripción | Ventajas | Desventajas |
|---------|-------------|----------|-------------|
| Oferta fija | Tope preestablecido | Simple | Dificultad creciente para los que llegan tarde, injusto |
| Cuota periódica | Cantidad fija de acuñación por período | Controla la oferta total | Las contribuciones se convierten en un juego de suma cero |
| **Sin tope + autoequilibrio por decaimiento** | Acuñar según demanda, el decaimiento quema automáticamente | Justo, sin desventaja para los que llegan tarde | Requiere un modelo de decaimiento preciso |

### Elección: Acuñación sin tope + Autoequilibrio por decaimiento

Justificación:
- Merit no es una moneda; no necesita escasez para mantener su valor
- Representa el "nivel de contribución activa actual"; el decaimiento lo garantiza
- Evita desventajas injustas para los que llegan tarde
- El poder de voto se basa en la participación; los cambios en la oferta total no afectan la equidad de la gobernanza

## 13.3 Por qué no ocurrirá la sobreemisión

> Pregunta clave planteada durante la discusión: Merit se crea de la nada — ¿no se sobreemitirá?

Respuesta:
1. **El decaimiento es un mecanismo natural de quema**: el MeriToken antiguo decae continuamente
2. **Equilibrio dinámico**: cuando la tasa de acuñación = tasa de decaimiento, la oferta total tiende hacia la estabilidad
3. **La participación determina el poder de voto**: incluso si la oferta total aumenta, el poder de voto individual depende de la participación en lugar del valor absoluto
4. **Analogía**: los conteos de citas académicas no tienen tope, pero la influencia de los artículos más antiguos decae naturalmente — el sistema se autoequilibra

## 13.4 Equilibrio dinámico

### Estado estacionario

Cuando el número de usuarios es estable: MeriToken total de la red ≈ constante

### Fase de crecimiento

Nuevos usuarios aumentan → la oferta total crece → pero el per cápita tiende hacia la estabilidad → las participaciones de poder de voto se diluyen naturalmente

### Fase de declive

Los usuarios activos disminuyen → la acuñación disminuye mientras el decaimiento continúa → la oferta total cae → las participaciones de los usuarios activos restantes aumentan

## 13.5 Asignación inicial

- El registro otorga MeriToken = e ≈ 2.718
- minMerit inicial = e
- Asegura que cada nuevo usuario tenga capacidad básica de participación
- e es lo suficientemente pequeño como para no diluir significativamente a los usuarios existentes, pero lo suficientemente grande como para garantizar derechos básicos de participación

### Arranque en frío opcional (unidireccional, con tope, decreciente)

Para resolver la voz del momento presente y el problema de arranque en frío, un participante PUEDE opcionalmente realizar un pago fiat unidireccional para recibir una concesión de reputación inicial por encima de la línea base, **hasta un tope rígido**. Este es el **único** punto de entrada monetario del sistema, y es estrictamente unidireccional:

- **Sin salida**: no existe una ruta reputación→fiat — sin reventa, sin liquidación, sin mercado secundario, sin transferencia.
- **Decae como todo Merit**: la reputación inicial comprada está sujeta al mismo decaimiento y colapsa hacia el valor mínimo (`minMerit`). PUEDE configurarse para decaer más rápido y/o ser no renovable, para evitar "seguir pagando = poder permanente".
- **Efecto neto**: una compra solo adquiere una ventaja temporal. Sin contribución genuina posterior, la reputación comprada decae necesariamente hacia el valor mínimo — el dinero no puede comprar voz persistente. En estado estacionario, la voz se determina por la contribución sostenida.

> ⚠️ Parámetros por especificar (solo alineación cualitativa): el tope de compra; si la reputación comprada decae más rápido o es no renovable; el acoplamiento exacto al modelo de decaimiento/valor mínimo. Véase la ADR `decisions/0001` de iFay (Arranque Decreciente). Esto **refina**, no revierte, el posicionamiento de MeriToken como "medida de contribución, no activo financiero".

## 13.6 Análisis de incentivos

MeriToken no es negociable, pero los incentivos que proporciona son:

| Incentivo | Descripción |
|-----------|-------------|
| Poder de voto | Influencia en la toma de decisiones comunitarias |
| Reconocimiento social | Alto MeriToken = alta credibilidad |
| Acceso prioritario | Asignación preferencial de ciertos recursos u oportunidades |
| Valor de legado | Puede transmitirse parcialmente a los descendientes |

En la era post-monetaria, el reconocimiento social y el poder de voto son en sí mismos los incentivos más fuertes.

## 13.7 Notas de discusión

> Perspectivas centrales del modelo económico:
> - MeriToken no es una moneda y no necesita las restricciones de la economía monetaria
> - El decaimiento es el mecanismo de "quema" más elegante — no se necesita intervención humana, se autoequilibra naturalmente
> - El poder de voto basado en la participación significa que los cambios en la oferta total no afectan la equidad de la gobernanza
> - La ventaja central de este modelo: simplicidad, autoequilibrio, equidad
> - No se necesita una "política monetaria" compleja para mantener la estabilidad
