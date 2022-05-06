import { ArgumentsHost, ExceptionFilter } from '@nestjs/common';
import { BusinessLogicException } from '../exceptions/business-logic.exception';
import { WinstonLogger } from '../logger/winston.logger';
export declare class BusinessLogicFilter<T extends BusinessLogicException> implements ExceptionFilter {
    readonly logger?: WinstonLogger | undefined;
    constructor(logger?: WinstonLogger | undefined);
    catch(exception: T, host: ArgumentsHost): void;
}
