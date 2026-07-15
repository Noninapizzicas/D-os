# Wrapper HTTP de Playwright (marcha larga del puente).
#
# Parte de la imagen oficial de Playwright, que YA trae los navegadores y sus
# dependencias de sistema — no hace falta `playwright install` a mano. Si ya
# tienes un contenedor Playwright, embute este server.js dentro en lugar de
# construir esta imagen (el contrato no cambia: es cable).
FROM mcr.microsoft.com/playwright:v1.48.0-jammy

WORKDIR /app
COPY package.json ./
RUN npm install --omit=dev
COPY server.js ./

ENV PORT=8100
EXPOSE 8100
CMD ["node", "server.js"]
