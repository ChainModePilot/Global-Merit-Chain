# Capítulo 6: Mecanismo de reconocimiento de contribuciones

## 6.1 El desafío central del reconocimiento

El reconocimiento de contribuciones es el componente más crítico y más difícil de GMC. El desafío central radica en:

- Las contribuciones pueden ser objetivas (cuantificables) o subjetivas (que requieren evaluación)
- La medición objetiva es naturalmente resistente al fraude pero tiene cobertura limitada
- La evaluación subjetiva tiene amplia cobertura pero es fácilmente manipulable (similar a las reseñas falsas en línea)

## 6.2 Dos métodos de adquisición

### Método 1: Medición objetiva

Basado en métricas objetivas verificables, el sistema acuña Merit automáticamente:

| Dimensión de medición | Ejemplos | Características |
|-----------------------|----------|-----------------|
| Por volumen | Clientes atendidos, propuestas entregadas | Auditable, resistente al fraude |
| Por tiempo | Horas de servicio, duración en línea | Las marcas de tiempo son verificables |
| Por producción | Commits de código, documentación producida | Rastreable en cadena |

Ventajas: Automático, eficiente, alta dificultad de fraude.
Limitaciones: No puede cubrir todos los tipos de contribuciones.

### Método 2: Recompensa por tarea

Merit preestablecido para una tarea específica; tras la finalización, las partes interesadas votan para confirmar:

1. **Publicar**: Definir el objetivo de la tarea, la recompensa de Merit y la duración de influencia
2. **Ejecutar**: El ejecutor completa la tarea y envía los resultados
3. **Votar**: Las partes interesadas votan sobre si se cumplen los criterios
4. **Acuñar**: Tras la aprobación, el sistema acuña MeriToken

## 6.3 Mecanismo de partes interesadas

### Quiénes son las partes interesadas

Partes con un interés en una contribución determinada. Por ejemplo:
- La contribución de un coFay de consultoría gubernamental → votada colectivamente por sus usuarios
- Una contribución a un proyecto de código abierto → votada por los usuarios y colaboradores del proyecto

### Regla clave: Excluir a individuos de alta intimidad

Dado que GMC registra la red de relaciones sociales, el sistema puede:
1. Identificar individuos cuya intimidad con el contribuyente excede un umbral
2. Excluir a estos individuos del grupo de votantes
3. Seleccionar votantes entre las partes interesadas restantes

Este es el mecanismo central para prevenir que "los cercanos voten por los cercanos".

### Condiciones de aprobación por consenso

- Se establece un umbral de proporción (por ejemplo, mayoría de 2/3)
- El peso del voto está vinculado al MeriToken propio del votante
- Una vez superado el umbral, el sistema acuña automáticamente

## 6.4 Determinación de la duración de influencia

Cada reconocimiento de contribución también debe determinar la duración de influencia:

| Método de determinación | Escenario aplicable |
|------------------------|---------------------|
| Preestablecido por tipo de contribución | Medición objetiva (por ejemplo, interacción de servicio al cliente = 30 días) |
| Establecido por el publicador de la tarea | Recompensa por tarea |
| Decidido colectivamente por los votantes | Consenso comunitario |

La duración de influencia determina la tasa de decaimiento de ese lote de Merit.

## 6.5 Estrategias anti-fraude

> Pregunta central en discusión: La minería de Bitcoin es medición puramente objetiva, naturalmente resistente al fraude. Pero GMC incluye evaluación subjetiva — ¿cómo prevenimos las reseñas falsas?
>
> Enfoque: No eliminar la subjetividad, sino hacer que el costo del fraude supere con creces el beneficio.

Combinación de defensas:

1. **Exclusión por intimidad**: Excluir a votantes con relaciones cercanas al sujeto evaluado
2. **Ponderación por MeriToken**: Los votantes de alta reputación tienen más peso; los defraudadores deben primero acumular una reputación genuina sustancial
3. **Auditoría de comportamiento de voto**: Votar frecuentemente a favor de un sujeto específico → marcado como anómalo
4. **Muestreo aleatorio**: Seleccionar aleatoriamente votantes del grupo de partes interesadas para reducir la posibilidad de colusión
5. **Responsabilidad retroactiva**: Si se descubre fraude, puede abordarse retroactivamente a través del mecanismo de sanción

### Principio de diseño

> Descomponer las contribuciones en componentes objetivamente medibles tanto como sea posible, reduciendo la proporción de evaluación subjetiva:
> - Priorizar la medición objetiva (automática, eficiente, resistente al fraude)
> - La evaluación subjetiva se usa solo para escenarios que no pueden cuantificarse objetivamente
> - La evaluación subjetiva emplea múltiples capas de defensa para reducir el riesgo de fraude

## 6.6 Notas de discusión

> Compromisos de diseño en el reconocimiento de contribuciones:
> - Eficiencia vs. equidad: La medición objetiva es eficiente pero limitada; la evaluación subjetiva es integral pero susceptible a manipulación
> - Participación vs. calidad: Reducir el umbral de votación aumenta la participación pero puede reducir la calidad de la evaluación
> - Enfoque actual: "Objetivo primero + complemento subjetivo + defensa multicapa"
> - Pregunta extendida: ¿Cómo se crea Merit de la nada? → Ver el capítulo de Modelo Económico
