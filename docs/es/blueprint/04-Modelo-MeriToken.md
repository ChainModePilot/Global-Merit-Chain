# Capítulo 4: Modelo MeriToken

## 4.1 Visión general del modelo

MeriToken es la unidad de medición central de GMC. Su diseño debe responder a una pregunta clave: **¿Cómo puede la medición de contribuciones reflejar la actividad actual y al mismo tiempo respetar las contribuciones históricas?**

La respuesta es: decaimiento exponencial + valor mínimo distinto de cero.

## 4.2 Dos valores clave

Cada MeritPocket mantiene dos valores centrales:

- **curMerit** (MeriToken actual): El valor de medición de contribución en tiempo real; decae con el tiempo y crece con nuevas contribuciones
- **minMerit** (valor mínimo): El límite inferior del decaimiento, que representa el sedimento a largo plazo de las contribuciones históricas; solo aumenta (excepto bajo sanciones)

```
curMerit ≥ minMerit ≥ e (valor inicial)
```

## 4.3 Adquisición

MeriToken se adquiere a través de contribuciones; el sistema acuña nuevos Tokens:

| Método de adquisición | Descripción | Condición de activación |
|-----------------------|-------------|-------------------------|
| Medición objetiva | Calculado automáticamente basándose en métricas verificables | El sistema registra automáticamente el umbral alcanzado |
| Recompensa por tarea | Merit preestablecido para una tarea específica | Las partes interesadas votan para aprobar tras la finalización |
| Asignación inicial | Otorgado al registrarse en la red | Registro de identidad completado |

Valor inicial = e ≈ 2.718 (la constante natural, naturalmente alineada con el modelo de decaimiento exponencial).

## 4.4 Modelo de decaimiento

### Idea central

Cada lote de adquisición de Merit tiene una **duración de influencia** independiente. La duración de influencia refleja la temporalidad de esa contribución — una contribución con 100 días de influencia tiene su Merit completamente decaído en 100 días.

### Fórmula de decaimiento por lote individual

```
MeriToken_i(t) = (V_i - B_i) × e^(-λ_i × t) + B_i
```

- `V_i`: Valor inicial de Merit del lote
- `B_i`: La contribución del lote al valor mínimo
- `λ_i`: Coeficiente de decaimiento, determinado por la duración de influencia T_i (λ_i = k / T_i, donde k es una constante)
- `t`: Tiempo transcurrido desde la adquisición

### MeriToken actual total

```
curMerit = Σ MeriToken_i(t)  (suma de todos los lotes activos)
```

Cuando todos los lotes han decaído completamente, curMerit se aproxima a minMerit.

## 4.5 Valor mínimo (minMerit)

### Regla de actualización

Cada vez que se adquiere nuevo Merit, el valor mínimo se actualiza:

Sea curMerit actual = M, Merit recién adquirido = x, valor mínimo actual = B, entonces:

```
Nuevo valor mínimo B' = (x + M) × B / M
```

Significado: El valor mínimo crece en proporción a la participación del nuevo Merit en el total.

### Propiedades

- Valor inicial = e ≈ 2.718
- Solo aumenta (excepto bajo sanciones)
- Representa el sedimento indeleble de las contribuciones históricas
- Incluso si las contribuciones cesan por completo, curMerit nunca caerá por debajo de minMerit

### Caso límite

Cuando curMerit = minMerit (es decir, en el estado mínimo) y se adquiere nuevo Merit x:
```
B' = (x + B) × B / B = x + B
```
El valor mínimo aumenta directamente en x — lo que significa que el Merit adquirido mientras se está en el estado mínimo se deposita íntegramente como valor mínimo.

## 4.6 Implementación del decaimiento independiente por lote

### Desafíos

- Cada MeritPocket debe mantener una lista de lotes de Merit
- Consultar el valor actual requiere iterar sobre todos los lotes que no han decaído completamente
- Los costos de almacenamiento y computación en cadena crecen linealmente con el número de lotes

### Estrategias de optimización

1. **Fusión de lotes**: Los lotes con duraciones de influencia similares se fusionan periódicamente para reducir el conteo de lotes activos
2. **Computación fuera de cadena**: Usar Rollup para calcular valores en tiempo real fuera de cadena; solo almacenar instantáneas y pruebas en cadena
3. **Sedimentación de lotes**: Cuando se excede el conteo máximo de lotes activos, los lotes más antiguos se sedimentan automáticamente en el valor mínimo
4. **Computación diferida**: Los valores precisos solo se calculan cuando son necesarios (por ejemplo, durante votaciones o consultas)

## 4.7 Filosofía de diseño

### ¿Por qué decaimiento exponencial?

- Incentiva la contribución continua en lugar de una única gran contribución seguida de inactividad
- Refleja la temporalidad de las contribuciones — las contribuciones más recientes tienen mayor impacto en la reputación actual
- Simula naturalmente el decaimiento de la memoria social
- Decae rápidamente al principio y se ralentiza después, alineándose con la intuición

### ¿Por qué un mínimo distinto de cero?

- Reconoce el valor a largo plazo de las contribuciones históricas — los esfuerzos pasados no se reducen completamente a cero
- Previene que los contribuyentes a largo plazo pierdan todo su poder de voto debido a una pausa breve
- El valor mínimo crece con las contribuciones acumuladas, recompensando la participación sostenida

### ¿Por qué duración de influencia independiente por lote?

- Diferentes contribuciones tienen naturalmente diferente temporalidad
- Una única interacción de servicio al cliente puede tener una influencia de solo 30 días
- Mantener un proyecto de código abierto puede tener una influencia que dure años
- Una tasa de decaimiento uniforme distorsionaría el valor de diferentes tipos de contribuciones

## 4.8 Notas de discusión

> Decisiones clave en el modelo MeriToken:
> - Decaimiento exponencial + mínimo distinto de cero: Logra un equilibrio entre "incentivar la participación continua" y "respetar las contribuciones históricas"
> - Duración de influencia independiente por lote: Aumenta la complejidad de implementación pero refleja con mayor precisión las diferencias en la temporalidad de las contribuciones
> - El valor mínimo solo aumenta (excepto bajo sanciones): Protege los derechos fundamentales de los contribuyentes a largo plazo
> - Valor inicial de e: Combina elegancia matemática con significado práctico
>
> Por examinar: Si la fórmula de actualización del valor mínimo se comporta razonablemente bajo condiciones extremas
