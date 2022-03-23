import { Body, Controller, Post } from '@nestjs/common';
import { VisitorService } from '../services/visitor.service';
import { ApiTags } from '@nestjs/swagger';
import { Criteria } from '../dto/criteria';
import { VisitorResponseDTO } from '../dto/visitorResponseDto';

@Controller('visitormanagement/v1/visitor/search')
@ApiTags('VisitorSearch')
export class VisitorController {
  constructor(public readonly visitor: VisitorService) {}

  @Post()
  getVisitorByUsername(@Body() crit: Criteria): Promise<VisitorResponseDTO> {
    return this.visitor.searchCriteria(crit);
  }
}
