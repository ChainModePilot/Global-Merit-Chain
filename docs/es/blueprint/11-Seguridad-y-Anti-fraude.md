# Capítulo 11: Seguridad y Anti-fraude

## 11.1 Modelo de amenazas

| Amenaza | Descripción | Impacto |
|---------|-------------|---------|
| Acumulación fraudulenta de Merit | Obtener MeriToken a través de contribuciones falsas | Poder de voto inflado |
| Votación colusiva | Múltiples partes conspirando para manipular votos de reconocimiento | Adquisición ilegítima de Merit |
| Fabricación de intimidad | Fabricar interacciones para aumentar la intimidad | Eludir exclusiones, reducir atenuación de herencia |
| Falsificación de identidad | Crear HumanIDs falsos | Múltiples identidades adquiriendo múltiples participaciones de Merit |
| Ataque Sybil | Una persona controlando múltiples identidades | Manipulación de votos |

## 11.2 Prevención de acumulación fraudulenta de Merit

### Salvaguardas para medición objetiva

- El sistema registra automáticamente, dejando poco margen para la manipulación humana
- La verificación cruzada es posible (por ejemplo, comparar horas de trabajo vs. producción)
- Detección estadística de anomalías

### Salvaguardas para evaluación subjetiva

> Principio central: hacer que el costo del fraude supere con creces el beneficio.

1. **Exclusión por intimidad**: excluir a votantes con relaciones cercanas
2. **Ponderación por MeriToken**: los votantes de alta reputación tienen más peso; los defraudadores deben primero acumular una reputación genuina sustancial
3. **Auditoría de comportamiento**: votar frecuentemente a favor de un individuo específico → marcado como anómalo
4. **Muestreo aleatorio**: seleccionar aleatoriamente votantes para reducir la posibilidad de colusión
5. **Responsabilidad retroactiva**: una vez descubierto el fraude, todos los participantes son sancionados

## 11.3 Prevención de fabricación de intimidad

- Evaluación de calidad de interacción (no solo frecuencia)
- Las interacciones unidireccionales son inválidas (deben ser bidireccionales)
- Grandes volúmenes de interacciones en un período corto se tratan como anómalos
- Interacciones aisladas de alta frecuencia entre dos individuos (sin círculo social compartido) se tratan como sospechosas

## 11.4 Seguridad de claves

- Esquemas de firma múltiple: las operaciones críticas requieren confirmación de múltiples claves
- Rotación de claves: reemplazo periódico
- Recuperación social: contactos de confianza asisten en la recuperación

## 11.5 Protección de privacidad

- El contenido de votación no es público (ZKP); solo se divulgan los resultados
- Los valores de intimidad pueden divulgarse selectivamente
- El contenido de interacción no se almacena en cadena
- Se soporta la participación anónima (ZKP demuestra elegibilidad sin revelar identidad)

## 11.6 Notas de discusión

> Filosofía de diseño del mecanismo de seguridad:
> - No existe una solución anti-fraude perfecta; el objetivo es hacer que el costo del fraude supere con creces el beneficio
> - Las defensas multicapa son más efectivas que cualquier mecanismo individual
> - Medidas preventivas + responsabilidad retroactiva forman un ciclo cerrado
> - El anti-fraude es un proceso adversarial continuo; el sistema debe poder evolucionar
