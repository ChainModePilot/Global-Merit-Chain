# Capítulo 7: Grafo de relaciones sociales e Intimidad

## 7.1 Por qué GMC necesita un grafo de relaciones sociales

GMC no solo registra contribuciones — también registra las relaciones entre personas. Esto no es una función adicional sino la base de mecanismos centrales:

| Mecanismo dependiente del grafo de relaciones | Propósito |
|----------------------------------------------|-----------|
| Mecanismo de herencia | Determina la proporción de atenuación (mayor intimidad = menor atenuación) |
| Exclusión de partes interesadas | Excluye a individuos demasiado cercanos al contribuyente durante la votación |
| Anti-fraude | Identifica patrones de relación anormales y comportamiento de colusión |
| Gobernanza comunitaria | Define los límites de la comunidad y las relaciones de membresía |

Sin el grafo de relaciones, ninguno de los mecanismos anteriores puede funcionar.

## 7.2 Fuentes de intimidad

La intimidad se deriva de las interacciones entre Fay y la red de relaciones sociales:

- **Frecuencia de interacción**: Frecuencia de comunicación y colaboración entre dos Fay
- **Profundidad de interacción**: Complejidad y duración de proyectos colaborativos
- **Declaraciones de relación**: Relaciones declaradas activamente por los usuarios (familia, colegas, etc.)
- **Participación conjunta**: Comunidades, proyectos y votaciones en las que participan juntos

## 7.3 Estrategia en cadena

### Por qué es necesario el almacenamiento en cadena

> Conclusión de las discusiones: Las relaciones sociales deben almacenarse en cadena para garantizar la autenticidad de las relaciones y prevenir la fabricación.
>
> Si los datos de relaciones pueden falsificarse, mecanismos como la atenuación de herencia y la exclusión de votación fallarán.

### Almacenamiento por capas

| Tipo de datos | Ubicación de almacenamiento | Justificación |
|---------------|----------------------------|---------------|
| Existencia de relación | En cadena | Garantiza la infalsificabilidad |
| Valores de intimidad | En cadena | Sirve como base para herencia y exclusión |
| Pruebas de cómputo de intimidad | En cadena (pruebas ZK) | Garantiza que el cómputo sea auditable |
| Detalles de interacción | Fuera de cadena | Gran volumen de datos, involucra privacidad |

### Anclaje de fuera de cadena a en cadena

- Los detalles de interacción se almacenan fuera de cadena
- Los resultados estadísticos se anclan periódicamente a la cadena mediante hash
- Se envían pruebas ZK cuando se actualiza la intimidad
- Cualquiera puede verificar que los datos fuera de cadena no han sido alterados mediante el hash

## 7.4 Modelo de intimidad

### Entradas de cómputo

```
Intimidad = f(frecuencia de interacción, profundidad de interacción, declaraciones de relación, participación conjunta, decaimiento temporal)
```

### Propiedades

- Tiene un límite superior máximo
- Decae con la falta prolongada de interacción
- El proceso de cómputo es auditable mediante pruebas en cadena
- Simetría por determinar (si A→B es igual a B→A)

### Mapeo de intimidad a función

| Rango de intimidad | Atenuación de herencia | Exclusión de votación |
|--------------------|------------------------|----------------------|
| Muy alta (> 0.9) | Mínima | Debe excluirse |
| Alta (0.7–0.9) | Baja | Se recomienda excluir |
| Media (0.4–0.7) | Moderada | No se excluye |
| Baja (0.1–0.4) | Alta | No se excluye |
| Muy baja (< 0.1) | Muy alta o no permitida | No se excluye |

## 7.5 Tipos de relación

- **Relaciones de sangre**: Padres, hijos, hermanos
- **Relaciones legales**: Cónyuge, tutor
- **Relaciones sociales**: Amigos, colegas, mentor-estudiante
- **Relaciones organizacionales**: Empleo, socios comerciales

Diferentes tipos de relación pueden tener diferentes líneas base de intimidad y tasas de decaimiento.

## 7.6 Anti-falsificación

- Las declaraciones de relación requieren confirmación de ambas partes (firmas bilaterales)
- Los registros de interacción son generados automáticamente por el sistema, no ingresados manualmente
- Un gran volumen de interacciones en un período corto se trata como anómalo
- Interacciones aisladas de alta frecuencia entre dos partes (sin círculo social compartido) se tratan como sospechosas
- Las relaciones deben estar ya en cadena antes de que ocurra un evento (no se permite el registro retroactivo para fines de herencia)

## 7.7 Protección de privacidad

- La existencia de relaciones es pública (utilizada para funciones públicas como la exclusión de votación)
- Los valores específicos de intimidad pueden divulgarse selectivamente
- Los detalles de interacción son estrictamente confidenciales
- Se utiliza ZKP para demostrar elegibilidad sin revelar relaciones específicas

## 7.8 Notas de discusión

> Consideraciones de diseño para el grafo de relaciones sociales:
> - Esta es la característica clave que distingue a GMC de un sistema de Token puro
> - El volumen de datos es el mayor desafío — un grafo social global es enormemente grande en escala
> - El almacenamiento por capas (relaciones en cadena + detalles fuera de cadena + pruebas de anclaje) es el enfoque equilibrado actual
> - La cuestión de simetría para la intimidad requiere mayor discusión
> - El grafo de relaciones en sí también requiere mecanismos anti-falsificación
