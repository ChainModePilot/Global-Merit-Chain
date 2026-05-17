# Capítulo 10: Herencia y Tutela

## 10.1 Contexto

Después de que un arquetipo humano fallece, su MeriToken acumulado y su iFay necesitan ser gestionados adecuadamente. La tensión central:

- Respetar las contribuciones históricas del fallecido
- Prevenir que individuos no relacionados obtengan un poder de voto desproporcionado
- Mantener el principio de que "la reputación no puede heredarse por derecho de nacimiento"

## 10.2 Reglas de herencia

### Heredable vs. No heredable

| Heredable | No heredable |
|-----------|--------------|
| MeriToken (con atenuación) | Identidad del iFay |
| Tutela del MeritPocket | El derecho a actuar bajo la identidad del fallecido |
| Propiedad del coFay | La vinculación entre iFay y arquetipo humano |

### Mecanismo de atenuación

```
MeriToken heredado = curMerit del fallecido × Coeficiente de herencia
Coeficiente de herencia = f(intimidad)  ← Mayor intimidad significa menor atenuación
```

- Los individuos con intimidad extremadamente baja no tienen permitido heredar
- El MeriToken heredado también decae normalmente
- La herencia aumenta el minMerit del heredero (pero el aumento también está sujeto a atenuación)

### Por qué es necesaria la atenuación

- MeriToken representa contribuciones personales; el heredero no es el creador
- La herencia sin atenuación llevaría a "reputación por derecho de nacimiento", violando los principios fundacionales de GMC
- La proporción de atenuación está vinculada a la intimidad: las relaciones cercanas en sí mismas reflejan contribución social
- MeriToken ya decae naturalmente; la atenuación de herencia además de eso asegura que la influencia se desvanezca rápidamente

## 10.3 Verificación de identidad del heredero

1. **Verificación de relación**: validada a través del grafo de relaciones sociales en cadena
2. **Confirmación de intimidad**: confirmar el valor y calcular la proporción de atenuación
3. **Testimonio multipartito**: contactos mutuos atestiguan y confirman
4. **Período de enfriamiento**: permite objeciones

### Prevención de fraude en herencia

- Las relaciones deben haber sido registradas en cadena durante la vida del fallecido
- No se permiten adiciones retroactivas
- La intimidad se basa en datos históricos de interacción y no puede fabricarse a corto plazo

## 10.4 Tutela

Tutela ≠ heredar identidad. Un tutor puede gestionar un iFay pero no puede actuar bajo la identidad del fallecido.

| Un tutor puede | Un tutor no puede |
|----------------|-------------------|
| Gestionar las operaciones diarias del iFay | Hacer declaraciones bajo la identidad del fallecido |
| Decidir si mover el iFay al cementerio digital | Votar bajo la identidad del fallecido |
| Manejar asuntos pendientes | Adquirir Merit bajo la identidad del fallecido |

Todas las acciones de tutela se marcan en cadena con el operador identificado como el tutor.

## 10.5 Cementerio digital

- Después de que un arquetipo humano fallece, su iFay puede ser trasladado al cementerio digital
- Las interacciones pasivas pueden seguir ocurriendo, pero se etiquetan como "desde el cementerio digital"
- No se genera activamente nuevo MeriToken
- El MeriToken existente continúa decayendo, aproximándose eventualmente a minMerit

## 10.6 Herencia de coFay

Como activo, coFay sigue la lógica de herencia de activos:
- La propiedad se transfiere al heredero
- El MeriToken no se atenúa (porque las contribuciones fueron generadas por el propio coFay)
- La distinción fundamental: lo que se hereda es "propiedad de activo", no "reputación personal"

## 10.7 Notas de discusión

> Filosofía de diseño del mecanismo de herencia:
> - Tensión central: respetar las contribuciones del fallecido vs. prevenir la reputación por derecho de nacimiento
> - Solución: permitir la herencia pero aplicar atenuación, con la proporción de atenuación determinada por la intimidad objetiva
> - La no transferibilidad de iFay garantiza el principio de que "la personalidad no puede heredarse"
> - El cementerio digital proporciona un marco para manejar el "legado digital" en la era de la IA
> - La herencia de coFay no tiene atenuación porque coFay es un activo, no una personalidad
