# Capítulo 12: Arquitectura Técnica

## 12.1 Visión general de la arquitectura

```
┌─────────────────────────────────────────────┐
│  Capa de aplicación (DApp, Interfaz Fay,     │
│  UI de gobernanza)                           │
├─────────────────────────────────────────────┤
│  Capa 2: ZK Rollup (Procesamiento de        │
│  transacciones de alta frecuencia)           │
│  - Registros de contribución, actualizaciones│
│    de intimidad, interacciones diarias       │
├─────────────────────────────────────────────┤
│  Capa 1: Cadena dedicada Substrate          │
│  (Liquidación y consenso)                    │
│  - Anclaje de raíz de estado, gestión de    │
│    identidad, votación de gobernanza         │
├─────────────────────────────────────────────┤
│  Capa de identidad: DID + PKI + ZKP          │
└─────────────────────────────────────────────┘
```

## 12.2 Comparación de selección tecnológica

> Se evaluaron múltiples enfoques durante la discusión:

| Enfoque | Ventajas | Desventajas | Conclusión |
|---------|----------|-------------|------------|
| Red principal de Ethereum | Ecosistema maduro, alta seguridad | Altas tarifas de gas, bajo TPS (15–30) | No adecuado para registro de alta frecuencia a escala poblacional |
| Ethereum L2 | Tarifas reducidas | Aún limitado por el ecosistema de Ethereum | Alternativa |
| DAG (IOTA/Nano) | Alto rendimiento, sin tarifas | Seguridad de consenso débil | Seguridad insuficiente |
| **Cadena personalizada Substrate** | Totalmente personalizable, sin tarifas de gas | Requiere construir ecosistema propio | **Recomendado** |

### El problema de las tarifas de gas

Las tarifas de gas son el costo computacional por transacción en cadenas públicas como Ethereum. Con toda la población generando grandes volúmenes de registros de micro-contribuciones diariamente, registrar cada uno en cadena sería prohibitivamente costoso. GMC requiere un método de registro gratuito o de costo extremadamente bajo.

### El problema del rendimiento

La red principal de Ethereum maneja aproximadamente 15–30 TPS. Para registros de contribución de miles de millones de usuarios en todo el mundo, este rendimiento está lejos de ser suficiente.

## 12.3 Cadena dedicada Substrate

### Por qué Substrate

1. **Consenso totalmente personalizable**: diseñar un algoritmo de consenso específicamente adecuado para el registro de contribuciones
2. **Sin tarifas de gas**: puede diseñarse para transacciones sin tarifas
3. **Módulos de gobernanza personalizables**: naturalmente adecuado para consenso comunitario
4. **Interoperabilidad con Polkadot**: puede interoperar con otras cadenas a través de cadenas de retransmisión
5. **Modular**: componer módulos de Runtime según sea necesario

### Justificación

> Los requisitos únicos de GMC hacen que las cadenas públicas de propósito general no sean adecuadas:
> - Participación de toda la población = volumen de transacciones extremadamente alto
> - Registros de micro-contribuciones = transacciones de alta frecuencia y bajo valor
> - No puede cobrar tarifas = registrar contribuciones no debe convertirse en una carga financiera
> - Requiere cálculos de decaimiento personalizados y algoritmos de intimidad

## 12.4 ZK Rollup

### Concepto central

Ejecución fuera de cadena, verificación en cadena:
- Los registros diarios de contribución se procesan a alta velocidad en L2, sin tarifas y con alto rendimiento
- Las pruebas de conocimiento cero de registros por lotes se envían periódicamente a L1
- L1 solo almacena raíces de estado comprimidas

### ZK Rollup vs. Optimistic Rollup

| Característica | ZK Rollup | Optimistic Rollup |
|----------------|-----------|-------------------|
| Método de verificación | Pruebas de conocimiento cero (garantía matemática) | Pruebas de fraude (período de desafío) |
| Tiempo de confirmación | Rápido | Lento (típicamente 7 días) |
| Seguridad | Garantía matemática | Depende de validadores honestos |
| Costo computacional | Alto | Bajo |

**Elección: ZK Rollup** — un sistema de reputación requiere confirmación rápida y seguridad con garantía matemática.

### División de responsabilidades

- **Procesamiento L2**: creación de registros de contribución, cálculo de MeriToken en tiempo real, actualizaciones de intimidad
- **Anclaje L1**: raíces de estado, registro/cambios de identidad, resultados de votación de gobernanza, registros de sanción

## 12.5 Almacenamiento de datos

```
En cadena (L1): Registro de identidad, raíces de estado, registros de gobernanza, registros de sanción
Rollup (L2): Saldos y lotes de MeriToken, intimidad, registros de contribución
Fuera de cadena (IPFS, etc.): Detalles de interacción, evidencia de contribución, archivos grandes
```

## 12.6 Mecanismo de consenso

- **Admisión de validadores**: requiere una cierta cantidad de MeriToken (colateral de reputación)
- **Incentivos de validación**: el trabajo de validación en sí es una contribución y puede ganar Merit
- **Consenso L1**: GRANDPA/BABE (valores predeterminados de Substrate)
- **Consenso L2**: BFT ligero

## 12.7 Estimaciones de rendimiento

Asumiendo 1.000 millones de usuarios, cada uno generando 5 registros por día:
- Volumen diario de transacciones: 5.000 millones de registros
- Requisito de TPS: ~58.000
- Requiere: múltiples instancias paralelas de Rollup (fragmentación), generación eficiente de pruebas, nodos L2 distribuidos

## 12.8 Notas de discusión

> Decisiones centrales en la arquitectura técnica:
> - Cadena dedicada en lugar de cadena pública de propósito general: los requisitos de GMC son demasiado especializados
> - ZK Rollup en lugar de Optimistic: requiere confirmación rápida y garantías matemáticas
> - Almacenamiento por capas: un equilibrio entre seguridad y escalabilidad
> - El rendimiento es el mayor desafío: la escala de participación de toda la población no tiene precedentes
>
> Este es un concepto de arquitectura en etapa de borrador de discusión; la implementación real deberá ajustarse según los desarrollos tecnológicos.
