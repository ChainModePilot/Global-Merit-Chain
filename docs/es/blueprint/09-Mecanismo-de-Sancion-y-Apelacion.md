# Capítulo 9: Mecanismo de Sanción y Apelación

## 9.1 Por qué se necesitan sanciones

Cualquier sistema de reputación requiere la capacidad de corregir errores. Cuando las contribuciones se reconocen incorrectamente o existe fraude, el sistema debe poder hacer correcciones.

El mecanismo de sanción es la salvaguarda última de la credibilidad de GMC.

## 9.2 Tipos de sanción

| Tipo | Efecto | Severidad |
|------|--------|-----------|
| Deducir curMerit | Reduce el MeriToken actual, afectando el poder de voto inmediato | Más leve |
| Deducir minMerit | Reduce el valor mínimo, afectando la garantía de reputación mínima a largo plazo | Severo |

Deducir minMerit es una sanción más severa — rompe la regla de que "el valor mínimo solo aumenta, nunca disminuye", lo que significa que la acumulación de contribuciones históricas se revoca parcialmente.

### Referencia de severidad

| Nivel de infracción | Método de sanción | Ejemplo |
|---------------------|-------------------|---------|
| Menor | Deducir curMerit parcial | Contribuciones exageradas |
| Moderado | Deducir curMerit significativo | Envíos duplicados |
| Severo | curMerit + minMerit parcial | Colusión para acumular Merit |
| Extremo | Deducción mayor de ambos | Fraude sistemático |

## 9.3 Proceso de activación

```
Queja presentada → Voto de aceptación de partes interesadas → [Rechazado si no se aprueba] → Voto de sanción → Ejecución
```

### Reglas

1. **Las quejas deben dirigirse a un lote específico de adquisición de Merit**: no se permiten quejas vagas; deben señalar un evento específico
2. **Aceptación de partes interesadas**: una cierta proporción de partes interesadas relevantes debe aceptar la queja antes de que se inicie una votación formal
3. **Voto de sanción**: requiere un umbral de aprobación más alto (por ejemplo, mayoría de 3/4)
4. **Ejecución automática**: una vez que el voto se aprueba, el sistema aplica automáticamente la deducción

### Prevención de quejas maliciosas

- Los denunciantes deben proporcionar evidencia o justificación
- Los denunciantes maliciosos frecuentes pueden ser marcados
- Los registros de quejas se almacenan en cadena, asegurando transparencia

## 9.4 Apelaciones

La parte sancionada tiene derecho a apelar:

1. Se puede presentar una apelación dentro de un período determinado después de la ejecución de la sanción
2. Un grupo más amplio de miembros de la comunidad vuelve a votar (para evitar que el mismo grupo juzgue repetidamente)
3. Si la apelación tiene éxito, la sanción se revoca y el MeriToken se restaura

## 9.5 Interacción con otros mecanismos

- **Las sanciones son el único mecanismo que puede reducir minMerit** (aparte del decaimiento natural)
- Los registros de sanción se almacenan en cadena, incluyendo la entidad sancionada, razón, cantidad y resultados de votación
- El historial de sanciones afecta la reputación social de la entidad (visible para otros)

## 9.6 Notas de discusión

> Filosofía de diseño del mecanismo de sanción:
> - Debe basarse en evidencia (dirigido a lotes específicos), previniendo "acusaciones infundadas"
> - Sanciones graduadas reflejan el principio de proporcionalidad
> - Las quejas requieren un umbral (aceptación de partes interesadas), previniendo el acoso malicioso
> - El derecho a apelar salvaguarda la equidad; ampliar el alcance previene efectos de cámara de eco
> - El hecho de que minMerit pueda ser reducido por sanciones sirve como el disuasivo más fuerte contra violaciones de integridad
