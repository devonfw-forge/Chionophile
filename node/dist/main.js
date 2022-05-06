"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const core_1 = require("@nestjs/core");
const app_module_1 = require("./app/app.module");
const winston_logger_1 = require("./app/shared/logger/winston.logger");
const common_1 = require("@nestjs/common");
const config_1 = require("@devon4node/config");
const helmet = require("helmet");
const swagger_1 = require("@nestjs/swagger");
const app_cluster_service_1 = require("./app-cluster.service");
async function bootstrap() {
    const app = await core_1.NestFactory.create(app_module_1.AppModule, { logger: new winston_logger_1.WinstonLogger() });
    const configModule = app.get(config_1.ConfigService);
    app.useGlobalPipes(new common_1.ValidationPipe({
        transform: true,
    }));
    app.setGlobalPrefix(configModule.values.globalPrefix);
    app.use(helmet());
    app.enableCors({
        origin: '*',
        credentials: true,
        exposedHeaders: 'Authorization',
        allowedHeaders: 'authorization, content-type',
    });
    if (configModule.values.isDev) {
        const options = new swagger_1.DocumentBuilder()
            .setTitle(configModule.values.swaggerConfig.swaggerTitle)
            .setDescription(configModule.values.swaggerConfig.swaggerDescription)
            .setVersion(configModule.values.swaggerConfig.swaggerVersion)
            .addBearerAuth()
            .build();
        const swaggerDoc = swagger_1.SwaggerModule.createDocument(app, options);
        swagger_1.SwaggerModule.setup((configModule.values.globalPrefix || '') + '/api', app, swaggerDoc);
    }
    await app.listen(configModule.values.port);
}
if (process.env.EXECUTION == 'cluster') {
    console.log('+++++++++++++++++++++++++++++');
    console.log('Executed with multiple cores');
    console.log('+++++++++++++++++++++++++++++');
    app_cluster_service_1.AppClusterService.clusterize(bootstrap);
}
else {
    console.log('+++++++++++++++++++++++++++++');
    console.log('Executed with single core');
    console.log('+++++++++++++++++++++++++++++');
    bootstrap();
}
//# sourceMappingURL=main.js.map